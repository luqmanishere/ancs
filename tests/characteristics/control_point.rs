#[cfg(test)]
mod control_point {
    use ancs::attributes::command::CommandID;
    use ancs::attributes::attribute::AttributeID;
    use ancs::characteristics::control_point::{GetNotificationAttributesRequest, GetAppAttributesRequest};

    #[test]
    fn struct_to_bytes() {
        let notification: GetNotificationAttributesRequest = GetNotificationAttributesRequest {
            command_id: CommandID::GetNotificationAttributes,
            notification_uid: 4294967295_u32,
            attribute_ids: vec![(AttributeID::AppIdentifier, None), (AttributeID::Title, Some(u16::MAX))],
        };

        let notification_bytes: Vec<u8> = notification.into();
        let expected_bytes: Vec<u8> = vec![0, 255, 255, 255, 255, 0, 1, 255, 255];

        assert_eq!(notification_bytes, expected_bytes)
    }

    #[test]
    fn bytes_to_struct() {
        let bytes: Vec<u8> = vec![0, 255, 255, 255, 255, 0, 1, 255, 255];
        let notification = GetNotificationAttributesRequest::parse(&bytes).unwrap();

        assert_eq!(notification.1.command_id, CommandID::GetNotificationAttributes);
        assert_eq!(notification.1.notification_uid, 4294967295_u32);
        assert_eq!(notification.1.attribute_ids, vec![(AttributeID::AppIdentifier, None), (AttributeID::Title, Some(u16::MAX))]);
    }
}
