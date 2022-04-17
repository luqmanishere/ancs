#[cfg(test)]
mod get_notification_attributes_request {
    use ancs::attributes::command::CommandID;
    use ancs::attributes::attribute::AttributeID;
    use ancs::characteristics::control_point::GetNotificationAttributesRequest;

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

#[cfg(test)]
mod get_app_attributes_request {
    use ancs::attributes::command::CommandID;
    use ancs::attributes::attribute::AttributeID;
    use ancs::characteristics::control_point::GetAppAttributesRequest;

    #[test]
    fn struct_to_bytes() {
        let notification: GetAppAttributesRequest = GetAppAttributesRequest {
            command_id: CommandID::GetNotificationAttributes,
            app_identifier: "Test".to_string(),
            attribute_ids: vec![AttributeID::AppIdentifier, AttributeID::Title],
        };

        let notification_bytes: Vec<u8> = notification.into();
        let expected_bytes: Vec<u8> = vec![0, 84, 101, 115, 116, 0, 0, 1];

        assert_eq!(notification_bytes, expected_bytes)
    }

    #[test]
    fn bytes_to_struct() {
        let bytes: Vec<u8> = vec![0, 84, 101, 115, 116, 0, 0, 1];
        let notification = GetAppAttributesRequest::parse(&bytes).unwrap();

        assert_eq!(notification.1.command_id, CommandID::GetNotificationAttributes);
        assert_eq!(notification.1.app_identifier, "Test");
        assert_eq!(notification.1.attribute_ids, vec![AttributeID::AppIdentifier, AttributeID::Title]);
    }
}

