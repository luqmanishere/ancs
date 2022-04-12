pub use crate::attributes::event::*;
pub use crate::attributes::category::*;

use nom::{
    number::complete::{be_u8},
    multi::{count},
    IResult,
};
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
