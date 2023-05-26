use bitflags::bitflags;
use nom::{
    number::complete::{le_u8},
    IResult, error::ParseError,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum EventID {
    NotificationAdded = 0,
    NotificationModified = 1,
    NotificationRemoved = 2,
}

impl From<EventID> for u8 {
    /// Converts a `EventID` to a `u8`
    /// 
    /// # Examples
    /// ```
    /// # use ancs::attributes::event::EventID;
    /// let data: u8 = EventID::NotificationAdded.into();
    /// 
    /// assert_eq!(0, data);
    /// ```
    fn from(original: EventID) -> u8 {
        match original {
            EventID::NotificationAdded => 0,
            EventID::NotificationModified => 1,
            EventID::NotificationRemoved => 2,
        }
    }
}

impl TryFrom<u8> for EventID {
    type Error = EventIDError;
    /// Attempts to convert a `u8` to a `EventID`
    /// 
    /// # Examples
    /// ```
    /// # use ancs::attributes::event::EventID;
    /// let event_id: EventID = EventID::try_from(0).unwrap();
    /// 
    /// assert_eq!(EventID::NotificationAdded, event_id);
    /// ```
    fn try_from(original: u8) -> Result<Self, Self::Error> {
        match original {
            0 => Ok(EventID::NotificationAdded),
            1 => Ok(EventID::NotificationModified),
            2 => Ok(EventID::NotificationRemoved),
            _ => Err(EventIDError),
        }
    }
}

impl EventID {
    /// Attempts to parse a `EventID` from a `&[u8]`
    /// 
    /// # Examples
    /// ```
    /// # use ancs::attributes::event::EventID;
    /// let data: [u8; 2] = [0, 1];
    /// let (data, event_id) = EventID::parse(&data).unwrap();
    /// 
    /// assert_eq!(EventID::NotificationAdded, event_id);
    /// ```
    /// 
    pub fn parse(i: &[u8]) -> IResult<&[u8], EventID> {
        let (i, event_id) = le_u8(i)?;

        match EventID::try_from(event_id) {
            Ok(event_id) => { Ok((i, event_id)) },
            Err(_) => Err(nom::Err::Failure(ParseError::from_error_kind(i, nom::error::ErrorKind::Fail))),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EventIDError;

bitflags! {
    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct EventFlag: u8 {
        const Silent = 0b00000001;
        const Important = 0b00000010;
        const PreExisting = 0b00000100;
        const PositiveAction = 0b00001000;
        const NegativeAction = 0b00010000;
    }
}

impl From<EventFlag> for u8 {
    /// Converts a `EventFlag` to a `u8`
    /// 
    /// # Examples
    /// ```
    /// # use ancs::attributes::event::EventFlag;
    /// let data: u8 = EventFlag::Silent.into();
    /// 
    /// assert_eq!(0b00000001, data);
    /// ```
    fn from(original: EventFlag) -> u8 {
        original.bits()
    }
}

impl TryFrom<u8> for EventFlag {
    type Error = EventFlagError;

    /// Attempts to convert a `u8` to a `EventFlag`
    /// 
    /// # Examples
    /// ```
    /// # use ancs::attributes::event::EventFlag;
    /// let event_flag: EventFlag = EventFlag::try_from(0b00000001).unwrap();
    /// 
    /// assert_eq!(EventFlag::Silent, event_flag);
    /// ```
    fn try_from(original: u8) -> Result<Self, Self::Error> {
        EventFlag::from_bits(original).ok_or(EventFlagError)
    }
}

impl EventFlag {
    /// Attempts to parse a `EventFlag` from a `&[u8]`
    /// 
    /// # Examples
    /// ```
    /// # use ancs::attributes::event::EventFlag;
    /// let data: [u8; 2] = [0b00000001, 0b00000010];
    /// let (data, event_flag) = EventFlag::parse(&data).unwrap();
    /// 
    /// assert_eq!(EventFlag::Silent, event_flag);
    /// ```
    /// 
    pub fn parse(i: &[u8]) -> IResult<&[u8], EventFlag> {
        let (i, event_flag) = le_u8(i)?;

        match EventFlag::try_from(event_flag) {
            Ok(event_flag) => { Ok((i, event_flag)) },
            Err(_) => Err(nom::Err::Failure(ParseError::from_error_kind(i, nom::error::ErrorKind::Fail))),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EventFlagError;
