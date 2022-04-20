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
    type Error = ();
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
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum EventFlag {
    Silent = 0b00000001,
    Important = 0b00000010,
    PreExisting = 0b00000100,
    PositiveAction = 0b00001000,
    NegativeAction = 0b00010000,
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
        match original {
            EventFlag::Silent => 0b00000001,
            EventFlag::Important => 0b00000010,
            EventFlag::PreExisting => 0b00000100,
            EventFlag::PositiveAction => 0b00001000,
            EventFlag::NegativeAction => 0b00010000,
        }
    }
}

impl TryFrom<u8> for EventFlag {
    type Error = ();

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
        match original.trailing_zeros() {
            0 => Ok(EventFlag::Silent),
            1 => Ok(EventFlag::Important),
            2 => Ok(EventFlag::PreExisting),
            3 => Ok(EventFlag::PositiveAction),
            4 => Ok(EventFlag::NegativeAction),
            _ => Err(()),
        }
    }
}
