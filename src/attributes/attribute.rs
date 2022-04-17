use nom::{
    multi::count,
    combinator::peek,
    number::complete::{le_u16, le_u8},
    IResult,
};

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
    pub fn parse(i: &[u8]) -> IResult<&[u8], AttributeID> {
        let (i, attribute_id) = le_u8(i)?;

        Ok((i, AttributeID::try_from(attribute_id).unwrap()))
    }

    pub fn is_sized(id: AttributeID) -> bool {
        match id {
            AttributeID::Title => true,
            AttributeID::Subtitle => true,
            AttributeID::Message => true,
            _ => false
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Attribute {
    pub id: AttributeID, 
    pub length: u16, 
    pub value: String
}

impl From<Attribute> for Vec<u8> {
    fn from(original: Attribute) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();

        let id: u8 = original.id.into();
        let length: [u8; 2] = original.length.to_le_bytes();
        let attribute: Vec<u8> = original.value.into_bytes();

        vec.push(id);
        vec.extend(length.to_vec());
        vec.extend(attribute);

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
                value: String::from_utf8(attribute).unwrap(),
            },
        ))
    }
}
