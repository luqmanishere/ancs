pub use crate::attributes::category::*;
pub use crate::attributes::event::*;

use nom::{multi::count, number::complete::le_u8, IResult};

pub const NOTIFICATION_SOURCE_UUID: &str = "9FBF120D-6301-42D9-8C58-25E699A21DBD";

#[derive(Debug, PartialEq, Clone)]
pub struct Notification {
    pub event_id: EventID,
    pub event_flags: EventFlag,
    pub category_id: CategoryID,
    pub category_count: u8,
    pub notification_uid: u32,
}

impl Notification {
    /// Attempts to parse a `Notification` from a `&[u8]`
    /// 
    /// # Examples
    /// ```
    /// # use ancs::attributes::category::CategoryID;
    /// # use ancs::attributes::event::EventFlag;
    /// # use ancs::attributes::event::EventID;
    /// # use ancs::characteristics::notification_source::Notification;
    /// let notification: Notification = Notification {
    ///     event_id: EventID::NotificationAdded,
    ///     event_flags: EventFlag::Silent,
    ///     category_id: CategoryID::Other,
    ///     category_count: 0,
    ///     notification_uid: 4294967295_u32,
    /// };
    ///
    /// let notification_bytes: [u8; 8] = notification.into();
    ///
    /// let parsed_notification = Notification::parse(&notification_bytes).unwrap().1;
    /// 
    /// assert_eq!(parsed_notification.event_id, EventID::NotificationAdded);
    /// assert_eq!(parsed_notification.event_flags, EventFlag::Silent);
    /// assert_eq!(parsed_notification.category_id, CategoryID::Other);
    /// assert_eq!(parsed_notification.category_count, 0);
    /// assert_eq!(parsed_notification.notification_uid, 4294967295_u32);
    /// ```
    pub fn parse(i: &[u8]) -> IResult<&[u8], Notification> {
        let (i, event_id) = le_u8(i)?;
        let (i, event_flags) = le_u8(i)?;
        let (i, category_id) = le_u8(i)?;
        let (i, category_count) = le_u8(i)?;
        let (i, notification_uid) = count(le_u8, 4)(i)?;

        Ok((
            i,
            Notification {
                event_id: EventID::try_from(event_id).unwrap(),
                event_flags: EventFlag::try_from(event_flags).unwrap(),
                category_id: CategoryID::try_from(category_id).unwrap(),
                category_count,
                notification_uid: u32::from_le_bytes(notification_uid.try_into().unwrap()),
            },
        ))
    }
}

impl From<Notification> for [u8; 8] {
    /// Converts a `Notification` to a `[u8; 8]`
    /// 
    /// # Examples
    /// ```
    /// # use ancs::attributes::category::CategoryID;
    /// # use ancs::attributes::event::EventFlag;
    /// # use ancs::attributes::event::EventID;
    /// # use ancs::characteristics::notification_source::Notification;
    /// let notification: Notification = Notification {
    ///    event_id: EventID::NotificationAdded,
    ///    event_flags: EventFlag::Silent,
    ///    category_id: CategoryID::Other,
    ///    category_count: 0,
    ///    notification_uid: 4294967295_u32,
    ///};
    ///
    ///let notification_bytes: [u8; 8] = notification.into();
    ///let expected_bytes: [u8; 8] = [0, 1, 0, 0, 255, 255, 255, 255];
    ///
    ///assert_eq!(notification_bytes, expected_bytes)
    /// ```

    fn from(original: Notification) -> [u8; 8] {
        let mut bytes: [u8; 8] = [0; 8];
        let uid_as_u8 = original.notification_uid.to_le_bytes();

        bytes[0] = original.event_id as u8;
        bytes[1] = original.event_flags as u8;
        bytes[2] = original.category_id as u8;
        bytes[3] = original.category_count as u8;

        bytes[4] = uid_as_u8[0];
        bytes[5] = uid_as_u8[1];
        bytes[6] = uid_as_u8[2];
        bytes[7] = uid_as_u8[3];

        return bytes;
    }
}
