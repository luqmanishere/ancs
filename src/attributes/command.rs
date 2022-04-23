use nom::{
    number::complete::{le_u8},
    IResult, error::ParseError,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CommandID {
    GetNotificationAttributes = 0,
    GetAppAttributes = 1,
    PerformNotificationAction = 2,
}

impl From<CommandID> for u8 {
    /// Converts a `CommandID` to a `u8` representaiton
    /// 
    /// # Examples
    /// ```
    /// # use ancs::attributes::command::CommandID;
    /// let data: u8 = CommandID::GetNotificationAttributes.into();
    /// 
    /// assert_eq!(0, data);
    /// ```
    fn from(original: CommandID) -> u8 {
        match original {
            CommandID::GetNotificationAttributes => 0,
            CommandID::GetAppAttributes => 1,
            CommandID::PerformNotificationAction => 2,
        }
    }
}

impl TryFrom<u8> for CommandID {
    type Error = CommandIDError;
    /// Attempts to convert a `u8` to a valid `CommandID`
    /// 
    /// # Examples
    /// ```
    /// # use ancs::attributes::command::CommandID;
    /// let command: CommandID = CommandID::try_from(0).unwrap();
    /// 
    /// assert_eq!(CommandID::GetNotificationAttributes, command);
    /// ```
    fn try_from(original: u8) -> Result<Self, Self::Error> {
        match original {
            0 => Ok(CommandID::GetNotificationAttributes),
            1 => Ok(CommandID::GetAppAttributes),
            2 => Ok(CommandID::PerformNotificationAction),
            _ => Err(CommandIDError),
        }
    }
}

impl CommandID {
    /// Attempts to parse a `CommandID` from a `&[u8]`
    /// 
    /// # Examples
    /// ```
    /// # use ancs::attributes::command::CommandID;
    /// let data: [u8; 2] = [0, 1];
    /// let (data, command_id) = CommandID::parse(&data).unwrap();
    /// 
    /// assert_eq!(CommandID::GetNotificationAttributes, command_id);
    /// ```
    pub fn parse(i: &[u8]) -> IResult<&[u8], CommandID> {
        let (i, command_id) = le_u8(i)?;

        match CommandID::try_from(command_id) {
            Ok(command_id) => { Ok((i, command_id)) },
            Err(_) => Err(nom::Err::Failure(ParseError::from_error_kind(i, nom::error::ErrorKind::Fail))),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CommandIDError;