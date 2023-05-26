use nom::{error::ParseError, number::complete::le_u8, IResult};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ActionID {
    Positive = 0,
    Negative = 1,
}

impl From<ActionID> for u8 {
    /// Convert a `ActionID` to a `u8`:
    ///
    /// # Examples
    /// ```
    /// # use ancs::attributes::action::ActionID;
    /// let data: u8 = ActionID::Positive.into();
    ///
    /// assert_eq!(0, data);
    /// ```
    fn from(original: ActionID) -> u8 {
        match original {
            ActionID::Positive => 0,
            ActionID::Negative => 1,
        }
    }
}

impl TryFrom<u8> for ActionID {
    type Error = ActionIDError;

    /// Attempts to convert a `u8` to a `ActionID`
    ///
    /// # Examples
    /// ```
    /// # use ancs::attributes::action::ActionID;
    /// let action_id: ActionID = ActionID::try_from(0).unwrap();
    ///
    /// assert_eq!(ActionID::Positive, action_id);
    /// ```
    ///
    fn try_from(original: u8) -> Result<Self, Self::Error> {
        match original {
            0 => Ok(ActionID::Positive),
            1 => Ok(ActionID::Negative),
            _ => Err(ActionIDError),
        }
    }
}

impl ActionID {
    /// Attempts to parse a `ActionID` from a `&[u8]`
    ///
    /// # Examples
    /// ```
    /// # use ancs::attributes::action::ActionID;
    /// let data: [u8; 2] = [0, 1];
    /// let (data, action_id) = ActionID::parse(&data).unwrap();
    ///
    /// assert_eq!(ActionID::Positive, action_id);
    /// ```
    ///
    pub fn parse(i: &[u8]) -> IResult<&[u8], ActionID> {
        let (i, action_id) = le_u8(i)?;

        match ActionID::try_from(action_id) {
            Ok(action_id) => Ok((i, action_id)),
            Err(_) => Err(nom::Err::Failure(ParseError::from_error_kind(
                i,
                nom::error::ErrorKind::Fail,
            ))),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ActionIDError;
