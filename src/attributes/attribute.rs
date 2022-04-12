use nom::{
    number::complete::{be_u8, be_u16},
    multi::{count},
    IResult,
};

pub enum AttributeID {
    AppIdentifier       = 0,
    Title               = 1,
    Subtitle            = 2,
    Message             = 3,
    MessageSize         = 4,
    Date                = 5,
    PositiveActionLabel = 6,
    NegativeActionLabel = 7,
}

impl From<AttributeID> for u8 {
    fn from(original: AttributeID) -> u8 {
        match original {
            AttributeID::AppIdentifier       => 0,
            AttributeID::Title               => 1,
            AttributeID::Subtitle            => 2,
            AttributeID::Message             => 3,
            AttributeID::MessageSize         => 4,
            AttributeID::Date                => 5,
            AttributeID::PositiveActionLabel => 6,
            AttributeID::NegativeActionLabel => 7,
        }
    }
}

impl TryFrom<u8> for AttributeID {
    type Error = ();

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
            _ => Err(())
        }
    }
}

impl AttributeID {
    pub fn parse(i:&[u8]) -> IResult<&[u8], AttributeID> {
        let (i, attribute_id) = be_u8(i)?;

        Ok((i, AttributeID::try_from(attribute_id).unwrap() ))
    }
}

pub struct AttributeList(AttributeID, u16, String);

impl From<AttributeList> for Vec<u8> {
    fn from(original: AttributeList) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();

        let id: u8 = original.0.into();
        let length: [u8; 2] = original.1.to_le_bytes();
        let mut attribute: Vec<u8> = original.2.into_bytes();

        vec.push(id);
        vec.append(&mut length.to_vec());
        vec.append(&mut attribute);

        return vec;
    }
}

impl AttributeList {
    pub fn parse(i: &[u8]) -> IResult<&[u8], AttributeList> {
        let (i, id) = AttributeID::parse(i)?;
        let (i, length) = be_u16(i)?;
        let (i, attribute) = count(be_u8, length.into())(i)?;

        Ok((i, AttributeList(AttributeID::try_from(id).unwrap(), length, String::from_utf8(attribute).unwrap())))
    }
}
