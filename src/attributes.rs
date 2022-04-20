//! ## Attributes
//! 
//! Attributes are the smallest data entity defined by `GATT`. In the case of the ANCS service
//! there are a wide variety of Attributes that can be used. Please see the attributes module
//! provided by this library for which attributes are valid when working with ANCS.
//! 
pub mod app;
pub mod category;
pub mod command;
pub mod event;
pub mod notification;

use nom::{
    multi::count,
    number::complete::{le_u16, le_u8},
    IResult,
};
use std::fmt::Debug;

/// The `Attribute` type. See [the module level documentation](index.html) for more.
#[derive(Debug, PartialEq, Clone)]
pub struct Attribute {
    pub id: notification::NotificationAttributeID, 
    pub length: u16, 
    pub value: Option<String>
}

impl From<Attribute> for Vec<u8> {
    /// Converts a `Attribute` to a `Vec<u8>`:
    /// 
    /// # Examples
    /// ```
    /// # use ancs::attributes::Attribute;
    /// # use ancs::attributes::notification::NotificationAttributeID;
    /// # let attribute_id = NotificationAttributeID::AppIdentifier;
    /// # let attribute_data = "test".to_string();
    /// # let attribute_length = attribute_data.as_bytes().len() as u16;
    /// let attribute: Attribute = Attribute {
    ///    id: attribute_id,
    ///    length: attribute_length,
    ///    value: Some(attribute_data)
    /// };
    /// 
    /// let converted_bytes: Vec<u8> = attribute.into();
    /// let expected_bytes: Vec<u8> = vec![0, 4, 0, 116, 101, 115, 116];
    /// 
    /// assert_eq!(expected_bytes, converted_bytes);
    /// ```
    fn from(original: Attribute) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();

        let id: u8 = original.id.into();
        let length: [u8; 2] = original.length.to_le_bytes();
        let attribute: Option<Vec<u8>> = match original.value {
            Some(value) => { Some(value.into_bytes()) },
            None => None,
        };

        vec.push(id);
        vec.extend(length.to_vec());

        // If the attribute's value isn't null we add it to our bytes.
        match attribute {
            Some(attribute) => { vec.extend(attribute)},
            None => (),
        };

        return vec;
    }
}

impl Attribute {
    /// Attempts to parse a `Attribute` from a `&[u8]`
    /// 
    /// # Examples
    /// 
    /// Convert a `Attribute` to a `Vec<u8>`:
    /// ```
    /// # use ancs::attributes::Attribute;
    /// # use ancs::attributes::notification::NotificationAttributeID;
    /// // Attribute Bytes Specificed by ANCS Standard
    /// let bytes: Vec<u8> = vec![0, 4, 0, 116, 101, 115, 116, 0];
    /// let (bytes, attribute) = Attribute::parse(&bytes).unwrap();
    /// 
    /// // Validate that all bytes were parsed per ANCS Standard
    /// assert_eq!(attribute.id, NotificationAttributeID::AppIdentifier);
    /// assert_eq!(attribute.length, 4);
    /// assert_eq!(attribute.value, Some("test".to_string()));
    /// 
    /// // Validate all remaining bytes are the same
    /// assert_eq!(bytes.len(), 1);
    /// assert_eq!(bytes, [0]);
    /// ```
    /// 
    pub fn parse(i: &[u8]) -> IResult<&[u8], Attribute> {
        let (i, id) = notification::NotificationAttributeID::parse(i)?;
        let (i, length) = le_u16(i)?;
        let (i, attribute) = count(le_u8, length.into())(i)?;

        Ok((
            i,
            Attribute {
                id: notification::NotificationAttributeID::try_from(id).unwrap(),
                length: length,
                value: Some(String::from_utf8(attribute).unwrap()),
            },
        ))
    }
}