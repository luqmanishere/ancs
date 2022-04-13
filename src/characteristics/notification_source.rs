pub use crate::attributes::event::*;
pub use crate::attributes::category::*;

use nom::{
    number::complete::{le_u8},
    multi::{count},
    IResult,
};

pub const NOTIFICATION_SOURCE_UUID: &str = "936DA01F9ABD4d9d80C702AF85C822A8";

#[derive(Debug, PartialEq, Clone)]
pub struct GattNotification {
    pub event_id: EventID,
    pub event_flags: EventFlag,
    pub category_id: CategoryID,
    pub category_count: u8,
    pub notification_uuid: [u8; 4],
}

impl GattNotification {
    pub fn parse(i:&[u8]) -> IResult<&[u8], GattNotification> {
        let (i, event_id) = le_u8(i)?;
        let (i, event_flags) = le_u8(i)?;
        let (i, category_id) = le_u8(i)?;
        let (i, category_count) = le_u8(i)?;
        let (i, notification_uuid) = count(le_u8, 4)(i)?;

        Ok((i, GattNotification { 
            event_id: EventID::try_from(event_id).unwrap(), 
            event_flags: EventFlag::try_from(event_flags).unwrap(), 
            category_id: CategoryID::try_from(category_id).unwrap(), 
            category_count, 
            notification_uuid: notification_uuid.try_into().unwrap()
        }))
    }
}

impl From<GattNotification> for [u8; 8] {
    fn from(original: GattNotification) -> [u8; 8] {
        let mut bytes: [u8; 8] = [0; 8];

        bytes[0] = original.event_id as u8;
        bytes[1] = original.event_flags as u8;
        bytes[2] = original.category_id as u8;
        bytes[3] = original.category_count as u8;

        bytes[4] = original.notification_uuid[0];
        bytes[5] = original.notification_uuid[1];
        bytes[6] = original.notification_uuid[2];
        bytes[7] = original.notification_uuid[3];

        return bytes;
    }
}
