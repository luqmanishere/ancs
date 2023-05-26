use crate::attributes::AppAttribute;
use crate::attributes::NotificationAttribute;
use crate::attributes::command::*;

use nom::combinator::all_consuming;
use nom::{
    bytes::complete::take_till,
    multi::{many0},
    number::complete::{le_u8, le_u32},
    sequence::{terminated},
    IResult,
};
use uuid::{uuid, Uuid};

pub const DATA_SOURCE_UUID: Uuid = uuid!("22EAC6E9-24D6-4BB5-BE44-B36ACE7C7BFB");

#[derive(Debug, PartialEq, Clone)]
pub struct GetNotificationAttributesResponse {
    pub command_id: CommandID,
    pub notification_uid: u32,
    pub attribute_list: Vec<NotificationAttribute>,
}

impl From<GetNotificationAttributesResponse> for Vec<u8> {
    /// Converts a `GetNotificationAttributesResponse` to a `Vec<u8>`
    /// 
    /// # Examples
    /// ```
    /// # use ancs::attributes::command::CommandID;
    /// # use ancs::attributes::NotificationAttribute;
    /// # use ancs::attributes::notification::NotificationAttributeID;
    /// # use ancs::characteristics::data_source::GetNotificationAttributesResponse;
    /// let notification: GetNotificationAttributesResponse = GetNotificationAttributesResponse {
    ///     command_id: CommandID::GetNotificationAttributes,
    ///     notification_uid: 4294967295_u32,
    ///     attribute_list: vec![
    ///         NotificationAttribute { 
    ///             id: NotificationAttributeID::AppIdentifier, 
    ///             length: "com.rust.test".to_string().as_bytes().len() as u16, 
    ///             value: Some("com.rust.test".to_string()) 
    ///         }
    ///     ],
    /// };
    ///
    /// let data: Vec<u8> = notification.into();
    /// let expected_data: Vec<u8> = vec![0, 255, 255, 255, 255, 0, 13, 0, 99, 111, 109, 46, 114, 117, 115, 116, 46, 116, 101, 115, 116];
    /// 
    /// assert_eq!(data, expected_data)
    /// ```
    fn from(original: GetNotificationAttributesResponse) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();

        // Convert all attributes to bytes
        let command_id: u8 = original.command_id.into();
        let notification_uid: [u8; 4] = original.notification_uid.to_le_bytes();
        let mut attribute_ids: Vec<u8> = original
            .attribute_list
            .into_iter()
            .map(|attribute| attribute.into())
            .into_iter()
            .flat_map(|att: Vec<u8>| att)
            .collect();

        vec.push(command_id);
        vec.append(&mut notification_uid.into());
        vec.append(&mut attribute_ids);

        return vec;
    }
}

impl GetNotificationAttributesResponse {
    /// Attempts to parse a `GetNotificationAttributesResponse` from a `&[u8]`
    /// 
    /// # Examples
    /// ```
    /// # use ancs::attributes::command::CommandID;
    /// # use ancs::attributes::NotificationAttribute;
    /// # use ancs::attributes::notification::NotificationAttributeID;
    /// # use ancs::characteristics::data_source::GetNotificationAttributesResponse;
    /// let bytes: Vec<u8> = vec![0, 255, 255, 255, 255, 0, 13, 0, 99, 111, 109, 46, 114, 117, 115, 116, 46, 116, 101, 115, 116];
    /// let notification = GetNotificationAttributesResponse::parse(&bytes).unwrap();
    ///
    /// assert_eq!(notification.1.command_id, CommandID::GetNotificationAttributes);
    /// assert_eq!(notification.1.notification_uid, 4294967295_u32);
    /// assert_eq!(notification.1.attribute_list, vec![
    ///    NotificationAttribute { 
    ///        id: NotificationAttributeID::AppIdentifier, 
    ///        length: "com.rust.test".to_string().as_bytes().len() as u16, 
    ///        value: Some("com.rust.test".to_string()) 
    ///    }
    /// ]);
    /// ```
    pub fn parse(i: &[u8]) -> IResult<&[u8], GetNotificationAttributesResponse> {
        let (i, command_id) = CommandID::parse(i)?;
        let (i, notification_uid) = le_u32(i)?;
        let (i, attribute_list) = all_consuming(many0(NotificationAttribute::parse))(i)?;

        Ok((
            i,
            GetNotificationAttributesResponse {
                command_id: command_id,
                notification_uid: notification_uid,
                attribute_list: attribute_list,
            },
        ))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct GetAppAttributesResponse {
    pub command_id: CommandID,
    pub app_identifier: String,
    pub attribute_list: Vec<AppAttribute>,
}

impl From<GetAppAttributesResponse> for Vec<u8> {
    /// Converts a `GetAppAttributesResponse` to a `Vec<u8>`
    /// 
    /// # Examples
    /// ```
    /// # use ancs::attributes::command::CommandID;
    /// # use ancs::attributes::AppAttribute;
    /// # use ancs::attributes::app::AppAttributeID;
    /// # use ancs::characteristics::data_source::GetAppAttributesResponse;
    /// 
    /// let response: GetAppAttributesResponse = GetAppAttributesResponse {
    ///     command_id: CommandID::GetAppAttributes,
    ///     app_identifier: "com.apple.test".to_string(),
    ///     attribute_list: vec![
    ///         AppAttribute { 
    ///             id: AppAttributeID::DisplayName, 
    ///             length: "Test".to_string().as_bytes().len() as u16, 
    ///             value: Some("Test".to_string()) 
    ///         }
    ///     ],
    /// };
    ///
    /// let expected_data: Vec<u8> = vec![
    ///     1,
    ///     99,
    ///     111, 
    ///     109, 
    ///     46,
    ///     97,
    ///     112,
    ///     112, 
    ///     108, 
    ///     101,
    ///     46,
    ///     116,
    ///     101,
    ///     115,
    ///     116,
    ///     0,
    ///     0,
    ///     4,
    ///     0,
    ///     84,
    ///     101,
    ///     115,
    ///     116
    /// ];
    /// let data: Vec<u8> = response.into();
    /// 
    /// assert_eq!(data, expected_data)
    /// ```
    fn from(original: GetAppAttributesResponse) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        
        // Convert all attributes to bytes
        let command_id: u8 = original.command_id.into();
        let mut app_identifier: Vec<u8> = original.app_identifier.as_bytes().to_vec();
        let mut attribute_ids: Vec<u8> = original
            .attribute_list
            .into_iter()
            .map(|attribute| attribute.into())
            .into_iter()
            .flat_map(|att: Vec<u8>| att)
            .collect();

        // Rust strings are not null terminated by default 
        // however it is possible that the user knows to insert
        // a null terminated string of some kind this helps us
        // ensure that all strings submitted to ANCS are null
        // terminated UTF-8 byte strings.
        if app_identifier.last().unwrap() != &0_u8 {
            app_identifier.push(0);
        }

        vec.push(command_id);
        vec.append(&mut app_identifier.into());
        vec.append(&mut attribute_ids);

        return vec;
    }
}

impl GetAppAttributesResponse {
    /// Attempts to parse a `GetAppAttributesResponse` from a `&[u8]`
    /// 
    /// # Examples
    /// ```
    /// # use ancs::attributes::command::CommandID;
    /// # use ancs::attributes::AppAttribute;
    /// # use ancs::attributes::app::AppAttributeID;
    /// # use ancs::characteristics::data_source::GetAppAttributesResponse;
    /// 
    /// let data: Vec<u8> = vec![
    ///     1,
    ///     99,
    ///     111, 
    ///     109, 
    ///     46,
    ///     97,
    ///     112,
    ///     112, 
    ///     108, 
    ///     101,
    ///     46,
    ///     116,
    ///     101,
    ///     115,
    ///     116,
    ///     0,
    ///     0,
    ///     4,
    ///     0,
    ///     84,
    ///     101,
    ///     115,
    ///     116
    /// ];
    /// 
    /// let (data, response) = GetAppAttributesResponse::parse(&data).unwrap();
    ///
    /// assert_eq!(response.command_id, CommandID::GetAppAttributes);
    /// assert_eq!(response.app_identifier, "com.apple.test");
    /// assert_eq!(response.attribute_list, vec![
    ///    AppAttribute { 
    ///        id: AppAttributeID::DisplayName, 
    ///        length: "Test".to_string().as_bytes().len() as u16, 
    ///        value: Some("Test".to_string()) 
    ///    }
    /// ]);
    /// ```
    pub fn parse(i: &[u8]) -> IResult<&[u8], GetAppAttributesResponse> {
        let (i, command_id) = CommandID::parse(i)?;
        let (i, app_identifier) = terminated(take_till(|b| b == 0), le_u8)(i)?;
        let (i, attribute_list) = all_consuming(many0(AppAttribute::parse))(i)?;

        Ok((
            i,
            GetAppAttributesResponse {
                command_id: command_id,
                app_identifier: String::from_utf8(app_identifier.to_vec()).unwrap(),
                attribute_list: attribute_list,
            },
        ))
    }
}
