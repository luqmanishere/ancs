#[cfg(test)]
mod attribute_id {
    use ancs::attributes::attribute::AttributeID;

    #[test]
    fn enum_to_u8() {
        let attribute_id: AttributeID = AttributeID::AppIdentifier;
        let u8_attribute_id: u8 = attribute_id.into();

        assert_eq!(u8::MIN, u8_attribute_id)
    }

    #[test]
    fn u8_to_enum() {
        let attribute_id: AttributeID = AttributeID::AppIdentifier;
        let enum_attribute_id: AttributeID = AttributeID::try_from(u8::MIN).unwrap();

        assert_eq!(attribute_id, enum_attribute_id);
    }

    #[test]
    fn parse_bytes() {
        let attribute_id: AttributeID = AttributeID::AppIdentifier;
        let attribute_id_byte: u8 = attribute_id.into();

        let bytes: [u8; 2] = [attribute_id_byte, 255];

        let parsed_attribute = AttributeID::parse(&bytes).unwrap();

        assert_eq!(parsed_attribute.0.len(), 1);
        assert_eq!(parsed_attribute.1, attribute_id);
    }
}

#[cfg(test)]
mod app_attribute_id {
    use ancs::attributes::attribute::AppAttributeID;

    #[test]
    fn enum_to_u8() {
        let attribute_id: AppAttributeID = AppAttributeID::DisplayName;
        let u8_attribute_id: u8 = attribute_id.into();

        assert_eq!(u8::MIN, u8_attribute_id)
    }

    #[test]
    fn u8_to_enum() {
        let attribute_id: AppAttributeID = AppAttributeID::DisplayName;
        let enum_attribute_id: AppAttributeID = AppAttributeID::try_from(u8::MIN).unwrap();

        assert_eq!(attribute_id, enum_attribute_id);
    }

    #[test]
    fn parse_bytes() {
        let attribute_id: AppAttributeID = AppAttributeID::DisplayName;
        let attribute_id_byte: u8 = attribute_id.into();

        let bytes: [u8; 2] = [attribute_id_byte, 255];

        let parsed_attribute = AppAttributeID::parse(&bytes).unwrap();

        assert_eq!(parsed_attribute.0.len(), 1);
        assert_eq!(parsed_attribute.1, attribute_id);
    }
}


#[cfg(test)]
mod attribute_list {
    use ancs::attributes::attribute::Attribute;
    use ancs::attributes::attribute::AttributeID;

    #[test]
    fn struct_to_u8() {
        // Create our fake attribute
        let attribute_id = AttributeID::AppIdentifier;
        let attribute_value = "test".to_string();
        let attribute_length = attribute_value.as_bytes().len() as u16;

        let attribute: Attribute = Attribute {
            id: attribute_id, 
            length: attribute_length,
            value: Some(attribute_value)
        };

        let attribute_bytes: Vec<u8> = attribute.clone().into();

        assert_eq!(u8::MIN, attribute_bytes[0]); // Identifier for attribute
        assert_eq!(4, attribute_bytes[1]); // Length of attribute
        assert_eq!(0, attribute_bytes[2]); // 4 doesn't need an extra byte so it's empty
        assert_eq!(116, attribute_bytes[3]); // t string char
        assert_eq!(101, attribute_bytes[4]); // e string char
        assert_eq!(115, attribute_bytes[5]); // s string char
        assert_eq!(116, attribute_bytes[6]); // t string char strings are not NULL terminated so this is the end
    }

    #[test]
    fn parse_bytes() {
        // Create our fake attribute
        let attribute_id = AttributeID::AppIdentifier;
        let attribute_data = "test".to_string();
        let attribute_length = attribute_data.as_bytes().len() as u16;

        let attribute: Attribute = Attribute {
            id: attribute_id,
            length: attribute_length,
            value: Some(attribute_data)
        };

        // Convert into bytes and then parse out of bytes
        let attribute_bytes: Vec<u8> = attribute.clone().into();
        let parsed_attribute = Attribute::parse(&attribute_bytes).unwrap();

        assert_eq!(attribute, parsed_attribute.1)
    }
}
