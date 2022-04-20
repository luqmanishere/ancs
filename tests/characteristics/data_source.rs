#[cfg(test)]
mod get_notification_attributes_response {
    use ancs::attributes::command::CommandID;
    use ancs::attributes::Attribute;
    use ancs::attributes::notification::NotificationAttributeID;
    use ancs::characteristics::data_source::GetNotificationAttributesResponse;

    #[test]
    fn struct_to_bytes() {
        let notification: GetNotificationAttributesResponse = GetNotificationAttributesResponse {
            command_id: CommandID::GetNotificationAttributes,
            notification_uid: 4294967295_u32,
            attribute_list: vec![
                Attribute { 
                    id: NotificationAttributeID::AppIdentifier, 
                    length: "com.rust.test".to_string().as_bytes().len() as u16, 
                    value: Some("com.rust.test".to_string()) 
                }
            ],
        };

        let notification_bytes: Vec<u8> = notification.into();
        let expected_bytes: Vec<u8> = vec![0, 255, 255, 255, 255, 0, 13, 0, 99, 111, 109, 46, 114, 117, 115, 116, 46, 116, 101, 115, 116];

        assert_eq!(notification_bytes, expected_bytes)
    }

    #[test]
    fn bytes_to_struct() {
        let bytes: Vec<u8> = vec![0, 255, 255, 255, 255, 0, 13, 0, 99, 111, 109, 46, 114, 117, 115, 116, 46, 116, 101, 115, 116];
        let notification = GetNotificationAttributesResponse::parse(&bytes).unwrap();

        assert_eq!(notification.1.command_id, CommandID::GetNotificationAttributes);
        assert_eq!(notification.1.notification_uid, 4294967295_u32);
        assert_eq!(notification.1.attribute_list, vec![
            Attribute { 
                id: NotificationAttributeID::AppIdentifier, 
                length: "com.rust.test".to_string().as_bytes().len() as u16, 
                value: Some("com.rust.test".to_string()) 
            }
        ],);
    }
}