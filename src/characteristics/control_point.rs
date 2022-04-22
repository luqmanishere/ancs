use crate::attributes::app::AppAttributeID;
use crate::attributes::notification::NotificationAttributeID;
use crate::attributes::command::*;

use nom::{
    bytes::complete::{take_till},
    combinator::{opt, fail},
    number::complete::le_u16,
    combinator::{verify},
    multi::{many0},
    number::complete::{le_u8, le_u32},
    branch::{alt},
    sequence::{pair, terminated},
    IResult,
};

pub const CONTROL_POINT_UUID: &str = "69D1D8F3-45E1-49A8-9821-9BBDFDAAD9D9";

pub struct GetNotificationAttributesRequest {
    pub command_id: CommandID,
    pub notification_uid: u32,
    // Rust doesn't have a clean way to express variadic tuples, and Apple has decided that some attributes need a max_length
    // assigned. To ensure that users can serialize requests without losing data we're left with this terrible solution of 
    // having users optionally provide a length or not I can't come up with a good name for a structure here so we're left with this.
    pub attribute_ids: Vec<(NotificationAttributeID, Option<u16>)>,
}

impl From<GetNotificationAttributesRequest> for Vec<u8> {
    /// Converts a `GetNotificationAttributesRequest` to a `Vec<u8>`
    /// 
    /// # Examples
    /// ```
    /// # use ancs::attributes::command::CommandID;
    /// # use ancs::attributes::notification::NotificationAttributeID;
    /// # use ancs::characteristics::control_point::GetNotificationAttributesRequest;
    /// let notification: GetNotificationAttributesRequest = GetNotificationAttributesRequest {
    ///    command_id: CommandID::GetNotificationAttributes,
    ///    notification_uid: 4294967295_u32,
    ///    attribute_ids: vec![(NotificationAttributeID::AppIdentifier, None), (NotificationAttributeID::Title, Some(u16::MAX))],
    /// };
    /// 
    /// let data: Vec<u8> = notification.into();
    /// let expected_data: Vec<u8> = vec![0, 255, 255, 255, 255, 0, 1, 255, 255];
    /// 
    /// assert_eq!(data, expected_data)
    /// ```
    fn from(original: GetNotificationAttributesRequest) -> Vec<u8> {
        let id = original.command_id as u8;
        let notification_uid: [u8; 4] = original.notification_uid.to_le_bytes();
        let mut attribute_ids: Vec<u8> = Vec::new();

        original.attribute_ids.into_iter().for_each(|id| {
            match id.1 {
                Some(length) => {
                    let byte: u8 = id.0.into();
                    let length_bytes: [u8; 2] = length.to_le_bytes();
                    attribute_ids.push(byte);
                    attribute_ids.extend(length_bytes);
                },
                None => {
                    let byte: u8 = id.0.into();
                    attribute_ids.push(byte);
                }
            };
        });

        let mut v: Vec<u8> = Vec::new();

        v.push(id);
        v.extend(notification_uid);
        v.append(&mut attribute_ids);

        return v;
    }
}

impl GetNotificationAttributesRequest {
    /// Attempts to parse a `GetNotificationAttributesRequest` from a `&[u8]`
    /// 
    /// # Examples
    /// ```
    /// # use ancs::attributes::command::CommandID;
    /// # use ancs::attributes::notification::NotificationAttributeID;
    /// # use ancs::characteristics::control_point::GetNotificationAttributesRequest;
    /// let data: Vec<u8> = vec![
    ///     0, 
    ///     255,
    ///     255, 
    ///     255, 
    ///     255, 
    ///     0, 
    ///     1, 
    ///     255, 
    ///     255
    /// ];
    /// let (data, notification) = GetNotificationAttributesRequest::parse(&data).unwrap();
    ///
    /// assert_eq!(notification.command_id, CommandID::GetNotificationAttributes);
    /// assert_eq!(notification.notification_uid, 4294967295_u32);
    /// assert_eq!(notification.attribute_ids, vec![(NotificationAttributeID::AppIdentifier, None), (NotificationAttributeID::Title, Some(u16::MAX))]);
    /// ```
    pub fn parse(i: &[u8]) -> IResult<&[u8], GetNotificationAttributesRequest> {
        let (i, command_id) = CommandID::parse(i)?;
        let (i, notification_uid) = le_u32(i)?;
        let (i, attribute_ids) = many0(
            alt((
                pair(
                    verify(NotificationAttributeID::parse, |&id| NotificationAttributeID::is_sized(id)),
                    opt(le_u16),
                ),
                pair(
                    verify(NotificationAttributeID::parse, |&id| !NotificationAttributeID::is_sized(id)),
                    opt(fail),
                ),
            ))
        )(i)?; 

        println!("{:?}", attribute_ids);

        Ok((
            i,
            GetNotificationAttributesRequest {
                command_id: command_id,
                notification_uid: notification_uid,
                attribute_ids: attribute_ids,
            },
        ))
    }
}

pub struct GetAppAttributesRequest {
    pub command_id: CommandID,
    pub app_identifier: String,
    pub attribute_ids: Vec<AppAttributeID>,
}

impl From<GetAppAttributesRequest> for Vec<u8> {
    /// Converts a `GetAppAttributesRequest` to a `Vec<u8>`
    /// 
    /// # Examples
    /// ```
    /// # use ancs::attributes::command::CommandID;
    /// # use ancs::attributes::app::AppAttributeID;
    /// # use ancs::characteristics::control_point::GetAppAttributesRequest;
    /// let notification: GetAppAttributesRequest = GetAppAttributesRequest {
    ///     command_id: CommandID::GetNotificationAttributes,
    ///     app_identifier: "com.apple.test".to_string(),
    ///     attribute_ids: vec![AppAttributeID::DisplayName],
    /// };
    ///
    /// let data: Vec<u8> = notification.into();
    /// let expected_data: Vec<u8> = vec![0, 99, 111, 109, 46, 97, 112, 112, 108, 101, 46, 116, 101, 115, 116, 0, 0];
    ///
    /// assert_eq!(data, expected_data)
    /// ```
    fn from(original: GetAppAttributesRequest) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();

        // Convert all attributes to bytes
        let command_id: u8 = original.command_id.into();
        let mut app_identifier: Vec<u8> = original.app_identifier.into_bytes();
        let mut attribute_ids: Vec<u8> = original
            .attribute_ids
            .into_iter()
            .map(|id| id.into())
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
        vec.append(&mut app_identifier);
        vec.append(&mut attribute_ids);

        return vec;
    }
}

impl GetAppAttributesRequest {
    /// Attempts to parse a `GetAppAttributesRequest` from a `&[u8]`
    /// 
    /// # Examples
    /// ```
    /// # use ancs::attributes::command::CommandID;
    /// # use ancs::attributes::app::AppAttributeID;
    /// # use ancs::characteristics::control_point::GetAppAttributesRequest;
    /// let data: Vec<u8> = vec![0, 84, 101, 115, 116, 0, 0];
    /// let (data, notification) = GetAppAttributesRequest::parse(&data).unwrap();
    ///
    /// assert_eq!(notification.command_id, CommandID::GetNotificationAttributes);
    /// assert_eq!(notification.app_identifier, "Test");
    /// assert_eq!(notification.attribute_ids, vec![AppAttributeID::DisplayName]);
    /// ```
    pub fn parse(i: &[u8]) -> IResult<&[u8], GetAppAttributesRequest> {
        let (i, command_id) = CommandID::parse(i)?;
        let (i, app_identifier) = terminated(take_till(|b| b == 0), le_u8)(i)?;
        let (i, attribute_ids) = many0(
            AppAttributeID::parse
        )(i)?; 

        println!("{:?}", attribute_ids);

        Ok((
            i,
            GetAppAttributesRequest {
                command_id: command_id,
                app_identifier: String::from_utf8(app_identifier.to_vec()).unwrap(),
                attribute_ids: attribute_ids,
            },
        ))
    }
}
