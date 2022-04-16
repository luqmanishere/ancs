#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CommandID {
    GetNotificationAttributes = 0,
    GetAppAttributes = 1,
    PerformNotificationAction = 2,
}

impl From<CommandID> for u8 {
    fn from(original: CommandID) -> u8 {
        match original {
            CommandID::GetNotificationAttributes => 0,
            CommandID::GetAppAttributes => 1,
            CommandID::PerformNotificationAction => 2,
        }
    }
}

impl TryFrom<u8> for CommandID {
    type Error = ();

    fn try_from(original: u8) -> Result<Self, Self::Error> {
        match original {
            0 => Ok(CommandID::GetNotificationAttributes),
            1 => Ok(CommandID::GetAppAttributes),
            2 => Ok(CommandID::PerformNotificationAction),
            _ => Err(()),
        }
    }
}
