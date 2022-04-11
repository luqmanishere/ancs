use std::convert::{
    Into,
    TryFrom,
};

use nom::{
    number::complete::{be_u8, be_u16},
    multi::{many0},
    bytes::complete::{take_until},
    multi::{count},
    IResult,
};

enum EventID {
    NotificationAdded = 0,
    NotificationModified = 1,
    NotificationRemoved = 2,
}

impl From<EventID> for u8 {
    fn from(original: EventID) -> u8 {
        match original {
            EventID::NotificationAdded    => 0,
            EventID::NotificationModified => 1,
            EventID::NotificationRemoved  => 2,
        }
    }
}

impl TryFrom<u8> for EventID {
    type Error = ();

    fn try_from(original: u8) -> Result<Self, Self::Error> {
        match original {
            0 => Ok(EventID::NotificationAdded),
            1 => Ok(EventID::NotificationModified),
            2 => Ok(EventID::NotificationRemoved),
            _ => Err(())
        }
    }
}

enum EventFlag {
    Silent         = 0b00000001,
    Important      = 0b00000010,
    PreExisting    = 0b00000100,
    PositiveAction = 0b00001000,
    NegativeAction = 0b00010000,
}

impl From<EventFlag> for u8 {
    fn from(original: EventFlag) -> u8 {
        match original {
            EventFlag::Silent         => 0b00000001,
            EventFlag::Important      => 0b00000010,
            EventFlag::PreExisting    => 0b00000100,
            EventFlag::PositiveAction => 0b00001000,
            EventFlag::NegativeAction => 0b00010000
        }
    }
}

impl TryFrom<u8> for EventFlag {
    type Error = ();

    fn try_from(original: u8) -> Result<Self, Self::Error> {
        match original {
            0b00000001 => Ok(EventFlag::Silent),
            0b00000010 => Ok(EventFlag::Important),
            0b00000100 => Ok(EventFlag::PreExisting),
            0b00001000 => Ok(EventFlag::PositiveAction),
            0b00010000 => Ok(EventFlag::NegativeAction),
            _ => Err(())
        }
    }
}

enum CategoryID {
    Other              = 0,
    IncomingCall       = 1,
    MissedCall         = 2,
    Voicemail          = 3,
    Social             = 4,
    Schedule           = 5,
    Email              = 6,
    News               = 7,
    HealthAndFitness   = 8,
    BusinessAndFinance = 9,
    Location           = 10,
    Entertainment      = 11,
}

impl From<CategoryID> for u8 {
    fn from(original: CategoryID) -> u8 {
        match original {
            CategoryID::Other              => 0,
            CategoryID::IncomingCall       => 1,
            CategoryID::MissedCall         => 2,
            CategoryID::Voicemail          => 3,
            CategoryID::Social             => 4,
            CategoryID::Schedule           => 5,
            CategoryID::Email              => 6,
            CategoryID::News               => 7,
            CategoryID::HealthAndFitness   => 8,
            CategoryID::BusinessAndFinance => 9,
            CategoryID::Location           => 10,
            CategoryID::Entertainment      => 11,
        }
    }
}

impl TryFrom<u8> for CategoryID {
    type Error = ();

    fn try_from(original: u8) -> Result<Self, Self::Error> {
        match original {
            0  => Ok(CategoryID::Other),
            1  => Ok(CategoryID::IncomingCall),
            2  => Ok(CategoryID::MissedCall),
            3  => Ok(CategoryID::Voicemail),
            4  => Ok(CategoryID::Social),
            5  => Ok(CategoryID::Schedule),
            6  => Ok(CategoryID::Email),
            7  => Ok(CategoryID::News),
            8  => Ok(CategoryID::HealthAndFitness),
            9  => Ok(CategoryID::BusinessAndFinance),
            10 => Ok(CategoryID::Location),
            11 => Ok(CategoryID::Entertainment),
            _  => Err(())
        }
    }
}

enum CommandID {
    GetNotificationAttributes = 0,
    GetAppAttributes          = 1,
    PerformNotificationAction = 2,
}

impl From<CommandID> for u8 {
    fn from(original: CommandID) -> u8 {
        match original {
            CommandID::GetNotificationAttributes => 0,
            CommandID::GetAppAttributes          => 1,
            CommandID::PerformNotificationAction => 2,
        }
    }
}

impl TryFrom<u8> for CommandID {
    type Error = ();

    fn try_from(original: u8) -> Result<Self, Self::Error> {
        match original {
            0 => Ok(CommandID::GetNotificationAttributes),
            1 => Ok(CommandID::GetAppAttributes),
            2 => Ok(CommandID::PerformNotificationAction),
            _ => Err(())
        }
    }
}

enum AttributeID {
    AppIdentifier       = 0,
    Title               = 1,
    Subtitle            = 2,
    Message             = 3,
    MessageSize         = 4,
    Date                = 5,
    PositiveActionLabel = 6,
    NegativeActionLabel = 7,
}

impl From<AttributeID> for u8 {
    fn from(original: AttributeID) -> u8 {
        match original {
            AttributeID::AppIdentifier       => 0,
            AttributeID::Title               => 1,
            AttributeID::Subtitle            => 2,
            AttributeID::Message             => 3,
            AttributeID::MessageSize         => 4,
            AttributeID::Date                => 5,
            AttributeID::PositiveActionLabel => 6,
            AttributeID::NegativeActionLabel => 7,
        }
    }
}

impl TryFrom<u8> for AttributeID {
    type Error = ();

    fn try_from(original: u8) -> Result<Self, Self::Error> {
        match original {
            0 => Ok(AttributeID::AppIdentifier),
            1 => Ok(AttributeID::Title),
            2 => Ok(AttributeID::Subtitle),
            3 => Ok(AttributeID::Message),
            4 => Ok(AttributeID::MessageSize),
            5 => Ok(AttributeID::Date),
            6 => Ok(AttributeID::PositiveActionLabel),
            7 => Ok(AttributeID::NegativeActionLabel),
            _ => Err(())
        }
    }
}

impl AttributeID {
    pub fn parse(i:&[u8]) -> IResult<&[u8], AttributeID> {
        let (i, attribute_id) = be_u8(i)?;

        Ok((i, AttributeID::try_from(attribute_id).unwrap() ))
    }
}

struct AttributeList(AttributeID, u16, String);

impl From<AttributeList> for Vec<u8> {
    fn from(original: AttributeList) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();

        let id: u8 = original.0.into();
        let mut length: [u8; 2] = original.1.to_le_bytes();
        let mut attribute: Vec<u8> = original.2.into_bytes();

        vec.push(id);
        vec.append(&mut length.to_vec());
        vec.append(&mut attribute);

        return vec;
    }
}

impl AttributeList {
    fn parse(i: &[u8]) -> IResult<&[u8], AttributeList> {
        let (i, id) = AttributeID::parse(i)?;
        let (i, length) = be_u16(i)?;
        let (i, attribute) = count(be_u8, length.into())(i)?;

        Ok((i, AttributeList(AttributeID::try_from(id).unwrap(), length, String::from_utf8(attribute).unwrap())))
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
    attribute_list: Vec<AttributeList>,
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
        let (i, attribute_list) = many0(AttributeList::parse)(i)?;

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
    attribute_list: Vec<AttributeList>,
}

impl GetAppAttributesResponse {
    pub fn parse(i:&[u8]) -> IResult<&[u8], GetAppAttributesResponse> {
        let (i, command_id) = be_u8(i)?;
        let (i, app_identifier) = take_until(" ")(i)?;
        let (i, attribute_list) = many0(AttributeList::parse)(i)?;

        Ok((i, GetAppAttributesResponse { command_id: CommandID::try_from(command_id).unwrap(), app_identifier: String::from_utf8(app_identifier.to_vec()).unwrap(), attribute_list: attribute_list } ))
    }
}



fn main() {
    println!("Hello, world!");
}