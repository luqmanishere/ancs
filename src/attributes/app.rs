use nom::{
    number::complete::{le_u8},
    IResult,
};

/// The `AppAttributeID` type. See [the module level documentation](index.html) for more.
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
    /// # use ancs::attributes::app::AppAttributeID;
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

    /// Attempts to convert a `u8` to a valid `AppAttributeID`
    /// 
    /// # Examples
    /// ```
    /// # use ancs::attributes::app::AppAttributeID;
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
    /// ```
    /// # use ancs::attributes::app::AppAttributeID;
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
