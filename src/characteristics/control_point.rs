pub use crate::attributes::attribute::*;
pub use crate::attributes::command::*;

use nom::{
    bytes::complete::take_until,
    number::complete::be_u16,
    combinator::{opt, peek, fail},
    number::complete::le_u16,
    combinator::{cond, map, verify, recognize},
    multi::{count, many0},
    number::complete::{le_u8, le_u32},
    branch::{alt},
    sequence::{pair, tuple, preceded},
    IResult,
};

pub const CONTROL_POINT_UUID: &str = "69D1D8F3-45E1-49A8-9821-9BBDFDAAD9D9";

pub struct GetNotificationAttributesRequest {
    pub command_id: CommandID,
    pub notification_uid: u32,
    // Rust doesn't have a clean way to express variadic tuples, and Apple has decided that some attributes need a max_length
    // assigned. To ensure that users can serialize requests without losing data we're left with this terrible solution of 
    // having users optionally provide a length or not I can't come up with a good name for a structure here so we're left with this.
    pub attribute_ids: Vec<(AttributeID, Option<u16>)>,
}

impl From<GetNotificationAttributesRequest> for Vec<u8> {
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
    pub fn parse(i: &[u8]) -> IResult<&[u8], GetNotificationAttributesRequest> {
        let (i, command_id) = le_u8(i)?;
        let (i, notification_uid) = le_u32(i)?;
        let (i, attribute_ids) = many0(
            alt((
                pair(
                    verify(AttributeID::parse, |&id| AttributeID::is_sized(id)),
                    opt(le_u16),
                ),
                pair(
                    verify(AttributeID::parse, |&id| !AttributeID::is_sized(id)),
                    opt(fail),
                ),
            ))
        )(i)?; 

        println!("{:?}", attribute_ids);

        Ok((
            i,
            GetNotificationAttributesRequest {
                command_id: CommandID::try_from(command_id).unwrap(),
                notification_uid: notification_uid,
                attribute_ids: attribute_ids,
            },
        ))
    }
}

pub struct GetAppAttributesRequest {
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
        let mut attribute_ids: Vec<u8> = original
            .attribute_ids
            .into_iter()
            .map(|id| id.into())
            .collect();

        vec.push(command_id);
        vec.append(&mut app_identifier);
        vec.append(&mut attribute_ids);

        return vec;
    }
}
