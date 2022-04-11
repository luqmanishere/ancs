use byteorder::{LittleEndian};
use nom::error::{FromExternalError, ParseError};
use nom::multi::many0;
use std::str;
use std::convert::{
    Into,
    TryFrom,
};
use nom::{
    number::complete::{be_u8, be_u16},
    bytes::complete::{take_until},
    multi::{count, fold_many0},
    IResult,
    Err,
    Parser,
    error::VerboseError
};

#[repr(u8)]
enum EventID {
    notification_added = 0,
    notification_modified = 1,
    notification_removed = 2,
}

impl TryFrom<u8> for EventID {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == EventID::notification_added as u8 => Ok(EventID::notification_added),
            x if x == EventID::notification_modified as u8 => Ok(EventID::notification_modified),
            x if x == EventID::notification_removed as u8 => Ok(EventID::notification_removed),
            _ => Err(()),
        }
    }
}

#[repr(u8)]
enum EventFlag {
    silent = 0b00000001,
    important = 0b00000010,
    pre_existing = 0b00000100,
    positive_action = 0b00001000,
    negative_action = 0b00010000,
}

impl TryFrom<u8> for EventFlag {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == EventFlag::silent as u8 => Ok(EventFlag::silent),
            x if x == EventFlag::important as u8 => Ok(EventFlag::important),
            x if x == EventFlag::pre_existing as u8 => Ok(EventFlag::pre_existing),
            x if x == EventFlag::positive_action as u8 => Ok(EventFlag::positive_action),
            x if x == EventFlag::negative_action as u8 => Ok(EventFlag::negative_action),
            _ => Err(()),
        }
    }
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

impl TryFrom<u8> for CategoryID {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == CategoryID::other as u8 => Ok(CategoryID::other),
            x if x == CategoryID::incoming_call as u8 => Ok(CategoryID::incoming_call),
            x if x == CategoryID::missed_call as u8 => Ok(CategoryID::missed_call),
            x if x == CategoryID::voicemail as u8 => Ok(CategoryID::voicemail),
            x if x == CategoryID::social as u8 => Ok(CategoryID::social),
            x if x == CategoryID::schedule as u8 => Ok(CategoryID::schedule),
            x if x == CategoryID::email as u8 => Ok(CategoryID::email),
            x if x == CategoryID::news as u8 => Ok(CategoryID::news),
            x if x == CategoryID::health_and_fitness as u8 => Ok(CategoryID::health_and_fitness),
            x if x == CategoryID::business_and_finance as u8 => Ok(CategoryID::business_and_finance),
            x if x == CategoryID::location as u8 => Ok(CategoryID::location),
            x if x == CategoryID::entertainment as u8 => Ok(CategoryID::entertainment),

            _ => Err(()),
        }
    }
}

#[repr(u8)]
enum CommandID {
    get_notification_attributes = 0,
    get_app_attributes = 1,
    perform_notification_action = 2,
}

impl TryFrom<u8> for CommandID {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == CommandID::get_notification_attributes as u8 => Ok(CommandID::get_notification_attributes),
            x if x == CommandID::get_app_attributes as u8 => Ok(CommandID::get_app_attributes),
            x if x == CommandID::perform_notification_action as u8 => Ok(CommandID::perform_notification_action),
            _ => Err(()),
        }
    }
}

struct Attribute(AttributeID, u16, String);

impl Attribute {
    fn parse(i: &[u8]) -> IResult<&[u8], Attribute> {
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

impl AttributeID {
    pub fn parse(i:&[u8]) -> IResult<&[u8], AttributeID> {
        let (i, attribute_id) = be_u8(i)?;

        Ok((i, AttributeID::try_from(attribute_id).unwrap() ))
    }
}



struct NotificationSourceCharacteristic {
    event_id: EventID,
    event_flags: EventFlag,
    category_id: CategoryID,
    category_count: u8,
    notification_uuid: Vec<u8>,
}

impl NotificationSourceCharacteristic {
    pub fn parse(i:&[u8]) -> IResult<&[u8], NotificationSourceCharacteristic> {
        let (i, event_id) = be_u8(i)?;
        let (i, event_flags) = be_u8(i)?;
        let (i, category_id) = be_u8(i)?;
        let (i, category_count) = be_u8(i)?;
        let (i, notification_uuid) = count(be_u8, 4)(i)?;

        Ok((i, NotificationSourceCharacteristic { event_id: EventID::try_from(event_id).unwrap(), event_flags: EventFlag::try_from(event_flags).unwrap(), category_id: CategoryID::try_from(category_id).unwrap(), category_count, notification_uuid } ))
    }
}

impl Into<[u8; 8]> for NotificationSourceCharacteristic {
    fn into(self) -> [u8; 8] {
        let mut packet: [u8; 8] = [0; 8];

        packet[0] = self.event_id as u8;
        packet[1] = self.event_flags as u8;
        packet[2] = self.category_id as u8;
        packet[3] = self.category_count;

        packet[4] = self.notification_uuid[0];
        packet[5] = self.notification_uuid[1];
        packet[6] = self.notification_uuid[2];
        packet[7] = self.notification_uuid[3];

        return packet;
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
        let notification_uuid: [u8; 4] = self.notification_uuid.to_le_bytes();
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
    notification_uuid: Vec<u8>,
    attribute_list: Vec<Attribute>,
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



fn main() {
    println!("Hello, world!");
}