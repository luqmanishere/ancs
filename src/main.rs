use byteorder::{LittleEndian};
use std::str;
use std::convert::{
    Into,
    TryFrom,
};
use nom::{
    number::complete::{be_u8, be_u16},
    multi::count,
    IResult,
};

#[repr(u8)]
enum EventID {
    notification_added = 0,
    notification_modified = 1,
    notification_removed = 2,
}

enum EventFlag {
    silent = 0b00000001,
    important = 0b00000010,
    pre_existing = 0b00000100,
    positive_action = 0b00001000,
    negative_action = 0b00010000,
}

#[repr(u8)]
enum CategoryID {
    other = 0,
    incoming_call = 1,
    missed_call = 2,
    voicemail = 3,
    social = 4,
    schedule = 5,
    email = 6,
    news = 7,
    health_and_fitness = 8,
    business_and_finance = 9,
    location = 10,
    entertainment = 11,
}

#[repr(u8)]
enum CommandID {
    get_notification_attributes = 0,
    get_app_attributes = 1,
    perform_notification_action = 2,
}

struct Attribute(AttributeID, u16, String);

impl Attribute {
    pub fn parse(i:&[u8]) -> IResult<&[u8], Self> {
        let (i, id) = be_u8(i)?;
        let (i, length) = be_u16(i)?;
        let (i, attribute) = count(be_u8, length.into())(i)?;

        Ok((i, Attribute(AttributeID::try_from(id).unwrap(), length, String::from_utf8(attribute).unwrap())))
    }
}

#[repr(u8)]
enum AttributeID {
    app_identifier = 0,
    title = 1,
    subtitle = 2,
    message = 3,
    message_size = 4,
    date = 5,
    positive_action_label = 6,
    negative_action_label = 7,
}

impl TryFrom<u8> for AttributeID {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == AttributeID::app_identifier as u8 => Ok(AttributeID::app_identifier),
            x if x == AttributeID::title as u8 => Ok(AttributeID::title),
            x if x == AttributeID::subtitle as u8 => Ok(AttributeID::subtitle),
            x if x == AttributeID::message as u8 => Ok(AttributeID::message),
            x if x == AttributeID::message_size as u8 => Ok(AttributeID::message_size),
            x if x == AttributeID::date as u8 => Ok(AttributeID::date),
            x if x == AttributeID::positive_action_label as u8 => Ok(AttributeID::positive_action_label),
            x if x == AttributeID::negative_action_label as u8 => Ok(AttributeID::negative_action_label),
            _ => Err(()),
        }
    }
}

struct NotificationSourceCharacteristic {
    event_id: EventID,
    event_flags: EventFlag,
    category_id: CategoryID,
    category_count: u8,
    notification_uuid: u32,
}

impl Into<Vec<u8>> for NotificationSourceCharacteristic {
    fn into(self) -> Vec<u8> {
        let id = self.event_id as u8;
    }
}

struct GetNotificationAttributesRequest {
    command_id: CommandID,
    notification_uuid: u32,
    attribute_ids: Vec<AttributeID>,
}

impl Into<Vec<u8>> for GetNotificationAttributesRequest {
    fn into(self) -> Vec<u8> {
        let id = self.command_id as u8;
        let notification_uuid = self.notification_uuid.to_le_bytes();
        let mut attribute_ids = self.attribute_ids.into_iter().map(|id| id as u8).collect();

        let mut v: Vec<u8> = Vec::new();
        
        v.push(id);
        v.extend(notification_uuid);
        v.append(&mut attribute_ids);

        return v;
    }
}

struct GetNotificationAttributesResponse {
    command_id: CommandID,
    notification_uuid: u32,
    attribute_list: Vec<Attribute>,
}

struct GetAppAttributesRequest {
    command_id: CommandID,
    app_identifier: String,
    attribute_ids: Vec<AttributeID>,
}

struct GetAppAttributesResponse {
    command_id: CommandID,
    app_identifier: String,
    attribute_list: Vec<Attribute>,
}



fn main() {
    println!("Hello, world!");
}