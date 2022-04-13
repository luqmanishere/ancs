pub use crate::attributes::attribute::*;
pub use crate::attributes::command::*;

use nom::{
    bytes::complete::{take_until},
    number::complete::{be_u8},
    multi::{count, many0},
    IResult,
};

pub const DATA_SOURCE_UUID: &str = "22EAC6E9-24D6-4BB5-BE44-B36ACE7C7BFB";

struct GetNotificationAttributesResponse {
    command_id: CommandID,
    notification_uuid: Vec<u8>,
    attribute_list: Vec<Attribute>,
}

impl From<GetNotificationAttributesResponse> for Vec<u8> {
    fn from(original: GetNotificationAttributesResponse) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();

        // Convert all attributes to bytes
        let command_id: u8 = original.command_id.into();
        let mut app_identifier: Vec<u8> = original.notification_uuid;
        let mut attribute_ids: Vec<u8> = original.attribute_list
            .into_iter()
            .map(|attribute| attribute.into())
            .into_iter()
            .flat_map(|att: Vec<u8>| att).collect();

        vec.push(command_id);
        vec.append(&mut app_identifier);
        vec.append(&mut attribute_ids);

        return vec;
    }
}

impl GetNotificationAttributesResponse {
    pub fn parse(i:&[u8]) -> IResult<&[u8], GetNotificationAttributesResponse> {
        let (i, command_id) = be_u8(i)?;
        let (i, notification_uuid) = count(be_u8, 4)(i)?;
        let (i, attribute_list) = many0(Attribute::parse)(i)?;

        Ok((i, GetNotificationAttributesResponse { command_id: CommandID::try_from(command_id).unwrap(), notification_uuid: notification_uuid, attribute_list: attribute_list } ))
    }
}

struct GetAppAttributesRequest {
    command_id: CommandID,
    app_identifier: String,
    attribute_ids: Vec<AttributeID>,
}

impl From<GetAppAttributesRequest> for Vec<u8> {
    fn from(original: GetAppAttributesRequest) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();

        // Convert all attributes to bytes
        let command_id: u8 = original.command_id.into();
        let mut app_identifier: Vec<u8> = original.app_identifier.into_bytes();
        let mut attribute_ids: Vec<u8> = original.attribute_ids.into_iter().map(|id| id.into()).collect();

        vec.push(command_id);
        vec.append(&mut app_identifier);
        vec.append(&mut attribute_ids);

        return vec;
    }
}

struct GetAppAttributesResponse {
    command_id: CommandID,
    app_identifier: String,
    attribute_list: Vec<Attribute>,
}

impl GetAppAttributesResponse {
    pub fn parse(i:&[u8]) -> IResult<&[u8], GetAppAttributesResponse> {
        let (i, command_id) = be_u8(i)?;
        let (i, app_identifier) = take_until(" ")(i)?;
        let (i, attribute_list) = many0(Attribute::parse)(i)?;

        Ok((i, GetAppAttributesResponse { command_id: CommandID::try_from(command_id).unwrap(), app_identifier: String::from_utf8(app_identifier.to_vec()).unwrap(), attribute_list: attribute_list } ))
    }
}

