#[cfg(test)]
mod category_id {
    use ancs::attributes::category::CategoryID;
    use ancs::attributes::event::EventFlag;
    use ancs::attributes::event::EventID;
    use ancs::characteristics::notification_source::GattNotification;

    #[test]
    fn struct_to_bytes() {
        let notification: GattNotification = GattNotification {
            event_id: EventID::NotificationAdded,
            event_flags: EventFlag::Silent,
            category_id: CategoryID::Other,
            category_count: 0,
            notification_uid: 4294967295_u32,
        };

        let notification_bytes: [u8; 8] = notification.into();
        let expected_bytes: [u8; 8] = [0, 1, 0, 0, 255, 255, 255, 255];

        assert_eq!(notification_bytes, expected_bytes)
    }

    #[test]
    fn bytes_to_struct() {
        let notification: GattNotification = GattNotification {
            event_id: EventID::NotificationAdded,
            event_flags: EventFlag::Silent,
            category_id: CategoryID::Other,
            category_count: 0,
            notification_uid: 4294967295_u32,
        };

        let notification_bytes: [u8; 8] = notification.into();

        let parsed_notification = GattNotification::parse(&notification_bytes).unwrap().1;

        assert_eq!(parsed_notification.event_id, EventID::NotificationAdded);
        assert_eq!(parsed_notification.event_flags, EventFlag::Silent);
        assert_eq!(parsed_notification.category_id, CategoryID::Other);
        assert_eq!(parsed_notification.category_count, 0);
        assert_eq!(parsed_notification.notification_uid, 4294967295_u32);
    }
}
