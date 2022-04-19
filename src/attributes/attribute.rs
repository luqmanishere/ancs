use nom::{
    multi::count,
    number::complete::{le_u16, le_u8},
    IResult,
};

/// The `AttributeID` type. See [the module level documentation](index.html) for more.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AttributeID {
    AppIdentifier = 0,
    Title = 1,
    Subtitle = 2,
    Message = 3,
    MessageSize = 4,
    Date = 5,
    PositiveActionLabel = 6,
    NegativeActionLabel = 7,
}

impl From<AttributeID> for u8 {
    /// Converts an `AttributeID` to its binary represenation
    /// 
    /// # Examples
    /// 
    /// Convert a `AttributeID` to a `u8`:
    /// ```
    /// use ancs::attributes::attribute::AttributeID;
    /// 
    /// let data: u8 = AttributeID::AppIdentifier.into();
    /// 
    /// assert_eq!(0, data);
    /// ```
    fn from(original: AttributeID) -> u8 {
        match original {
            AttributeID::AppIdentifier => 0,
            AttributeID::Title => 1,
            AttributeID::Subtitle => 2,
            AttributeID::Message => 3,
            AttributeID::MessageSize => 4,
            AttributeID::Date => 5,
            AttributeID::PositiveActionLabel => 6,
            AttributeID::NegativeActionLabel => 7,
        }
    }
}

impl TryFrom<u8> for AttributeID {
    type Error = ();

    /// Attempts to convert a u8 to a valid `AttributeID`
    /// 
    /// # Examples
    /// 
    /// ```
    /// use ancs::attributes::attribute::AttributeID;
    /// 
    /// let attribute: AttributeID = AttributeID::try_from(0).unwrap();
    /// 
    /// assert_eq!(AttributeID::AppIdentifier, attribute);
    /// ```
    /// 
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
            _ => Err(()),
        }
    }
}

impl AttributeID {
    /// Attempts to parse a AttributeID from a `&[u8]`
    /// 
    /// # Examples
    /// 
    /// ```
    /// use ancs::attributes::attribute::AttributeID;
    /// 
    /// let data: [u8; 2] = [0, 1];
    /// let (data, attribute) = AttributeID::parse(&data).unwrap();
    /// 
    /// assert_eq!(AttributeID::AppIdentifier, attribute);
    /// ```
    /// 
    pub fn parse(i: &[u8]) -> IResult<&[u8], AttributeID> {
        let (i, attribute_id) = le_u8(i)?;

        Ok((i, AttributeID::try_from(attribute_id).unwrap()))
    }

    /// Determines if an AttributeID should have a size associated
    /// based on what's outlined in the ANCS specification.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use ancs::attributes::attribute::AttributeID;
    /// 
    /// let app_identifier = AttributeID::AppIdentifier;
    /// let title = AttributeID::Title;
    /// 
    /// assert_eq!(AttributeID::is_sized(app_identifier), false);
    /// assert_eq!(AttributeID::is_sized(title), true);
    /// ```
    /// 
    pub fn is_sized(id: AttributeID) -> bool {
        match id {
            AttributeID::Title => true,
            AttributeID::Subtitle => true,
            AttributeID::Message => true,
            _ => false
        }
    }
}


#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AppAttributeID {
    DisplayName = 0,
}

impl From<AppAttributeID> for u8 {
    /// Converts an `AppAttributeID` to its binary represenation
    /// 
    /// # Examples
    /// 
    /// Convert a `AppAttributeID` to a `u8`:
    /// ```
    /// use ancs::attributes::attribute::AppAttributeID;
    /// 
    /// let data: u8 = AppAttributeID::DisplayName.into();
    /// 
    /// assert_eq!(0, data);
    /// ```
    fn from(original: AppAttributeID) -> u8 {
        match original {
            AppAttributeID::DisplayName => 0,
        }
    }
}

impl TryFrom<u8> for AppAttributeID {
    type Error = ();

    /// Attempts to convert a u8 to a valid `AppAttributeID`
    /// 
    /// # Examples
    /// 
    /// ```
    /// use ancs::attributes::attribute::AppAttributeID;
    /// 
    /// let attribute: AppAttributeID = AppAttributeID::try_from(0).unwrap();
    /// 
    /// assert_eq!(AppAttributeID::DisplayName, attribute);
    /// ```
    /// 
    fn try_from(original: u8) -> Result<Self, Self::Error> {
        match original {
            0 => Ok(AppAttributeID::DisplayName),
            _ => Err(()),
        }
    }
}

impl AppAttributeID {
    /// Attempts to parse a `AppAttributeID` from a `&[u8]`
    /// 
    /// # Examples
    /// 
    /// ```
    /// use ancs::attributes::attribute::AppAttributeID;
    /// 
    /// let data: [u8; 2] = [0, 1];
    /// let (data, attribute) = AppAttributeID::parse(&data).unwrap();
    /// 
    /// assert_eq!(AppAttributeID::DisplayName, attribute);
    /// ```
    /// 
    pub fn parse(i: &[u8]) -> IResult<&[u8], AppAttributeID> {
        let (i, attribute_id) = le_u8(i)?;

        Ok((i, AppAttributeID::try_from(attribute_id).unwrap()))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Attribute {
    pub id: AttributeID, 
    pub length: u16, 
    pub value: Option<String>
}

impl From<Attribute> for Vec<u8> {
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
    pub fn parse(i: &[u8]) -> IResult<&[u8], Attribute> {
        let (i, id) = AttributeID::parse(i)?;
        let (i, length) = le_u16(i)?;
        let (i, attribute) = count(le_u8, length.into())(i)?;

        Ok((
            i,
            Attribute {
                id: AttributeID::try_from(id).unwrap(),
                length: length,
                value: Some(String::from_utf8(attribute).unwrap()),
            },
        ))
    }
}
