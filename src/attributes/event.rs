#[derive(Debug, PartialEq, Clone, Copy)]
pub enum EventID {
    NotificationAdded = 0,
    NotificationModified = 1,
    NotificationRemoved = 2,
}

impl From<EventID> for u8 {
    fn from(original: EventID) -> u8 {
        match original {
            EventID::NotificationAdded    => 0,
            EventID::NotificationModified => 1,
            EventID::NotificationRemoved  => 2,
        }
    }
}

impl TryFrom<u8> for EventID {
    type Error = ();

    fn try_from(original: u8) -> Result<Self, Self::Error> {
        match original {
            0 => Ok(EventID::NotificationAdded),
            1 => Ok(EventID::NotificationModified),
            2 => Ok(EventID::NotificationRemoved),
            _ => Err(())
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum EventFlag {
    Silent         = 0b00000001,
    Important      = 0b00000010,
    PreExisting    = 0b00000100,
    PositiveAction = 0b00001000,
    NegativeAction = 0b00010000,
}

impl From<EventFlag> for u8 {
    fn from(original: EventFlag) -> u8 {
        match original {
            EventFlag::Silent         => 0b00000001,
            EventFlag::Important      => 0b00000010,
            EventFlag::PreExisting    => 0b00000100,
            EventFlag::PositiveAction => 0b00001000,
            EventFlag::NegativeAction => 0b00010000
        }
    }
}

impl TryFrom<u8> for EventFlag {
    type Error = ();

    fn try_from(original: u8) -> Result<Self, Self::Error> {
        match original {
            0b00000001 => Ok(EventFlag::Silent),
            0b00000010 => Ok(EventFlag::Important),
            0b00000100 => Ok(EventFlag::PreExisting),
            0b00001000 => Ok(EventFlag::PositiveAction),
            0b00010000 => Ok(EventFlag::NegativeAction),
            _ => Err(())
        }
    }
}
