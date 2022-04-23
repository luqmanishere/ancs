use nom::{
    number::complete::{le_u8},
    IResult, error::ParseError,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CategoryID {
    Other = 0,
    IncomingCall = 1,
    MissedCall = 2,
    Voicemail = 3,
    Social = 4,
    Schedule = 5,
    Email = 6,
    News = 7,
    HealthAndFitness = 8,
    BusinessAndFinance = 9,
    Location = 10,
    Entertainment = 11,
}

impl From<CategoryID> for u8 {
    /// Converts an `CategoryID` to its binary represenation
    /// 
    /// # Examples
    /// 
    /// Convert a `CategoryID` to a `u8`:
    /// ```
    /// # use ancs::attributes::category::CategoryID;
    /// let data: u8 = CategoryID::Other.into();
    /// 
    /// assert_eq!(0, data);
    /// ```
    fn from(original: CategoryID) -> u8 {
        match original {
            CategoryID::Other => 0,
            CategoryID::IncomingCall => 1,
            CategoryID::MissedCall => 2,
            CategoryID::Voicemail => 3,
            CategoryID::Social => 4,
            CategoryID::Schedule => 5,
            CategoryID::Email => 6,
            CategoryID::News => 7,
            CategoryID::HealthAndFitness => 8,
            CategoryID::BusinessAndFinance => 9,
            CategoryID::Location => 10,
            CategoryID::Entertainment => 11,
        }
    }
}

impl TryFrom<u8> for CategoryID {
    type Error = CategoryIDError;

    /// Attempts to convert a u8 to a valid `CategoryID`
    /// 
    /// # Examples
    /// 
    /// ```
    /// # use ancs::attributes::app::AppAttributeID;
    /// let attribute: AppAttributeID = AppAttributeID::try_from(0).unwrap();
    /// 
    /// assert_eq!(AppAttributeID::DisplayName, attribute);
    /// ```
    /// 
    fn try_from(original: u8) -> Result<Self, Self::Error> {
        match original {
            0 => Ok(CategoryID::Other),
            1 => Ok(CategoryID::IncomingCall),
            2 => Ok(CategoryID::MissedCall),
            3 => Ok(CategoryID::Voicemail),
            4 => Ok(CategoryID::Social),
            5 => Ok(CategoryID::Schedule),
            6 => Ok(CategoryID::Email),
            7 => Ok(CategoryID::News),
            8 => Ok(CategoryID::HealthAndFitness),
            9 => Ok(CategoryID::BusinessAndFinance),
            10 => Ok(CategoryID::Location),
            11 => Ok(CategoryID::Entertainment),
            _ => Err(CategoryIDError),
        }
    }
}

impl CategoryID {
    /// Attempts to parse a `CategoryID` from a `&[u8]`
    /// 
    /// # Examples
    /// ```
    /// # use ancs::attributes::category::CategoryID;
    /// let data: [u8; 2] = [0, 1];
    /// let (data, category_id) = CategoryID::parse(&data).unwrap();
    /// 
    /// assert_eq!(CategoryID::Other, category_id);
    /// ```
    /// 
    pub fn parse(i: &[u8]) -> IResult<&[u8], CategoryID> {
        let (i, category_id) = le_u8(i)?;

        match CategoryID::try_from(category_id) {
            Ok(category_id) => { Ok((i, category_id)) },
            Err(_) => Err(nom::Err::Failure(ParseError::from_error_kind(i, nom::error::ErrorKind::Fail))),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CategoryIDError;