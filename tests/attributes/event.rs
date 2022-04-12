#[cfg(test)]
mod event_id {
    use ancs::attributes::event::EventID;

    #[test]
    fn enum_to_u8() {
        let event_id: EventID = EventID::NotificationAdded;
        let u8_event_id: u8 = event_id.into();

        assert_eq!(u8::MIN, u8_event_id)
    }

    #[test]
    fn u8_to_enum() {
        let event_id: EventID = EventID::NotificationAdded;
        let enum_event_id: EventID = EventID::try_from(u8::MIN).unwrap();

        assert_eq!(event_id, enum_event_id);
    }
}

#[cfg(test)]
mod event_flag {
    use ancs::attributes::event::EventFlag;

    #[test]
    fn enum_to_u8() {
        let event_flag: EventFlag = EventFlag::Important;
        let u8_event_flag: u8 = event_flag.into();

        assert_eq!(0b00000010, u8_event_flag)
    }

    #[test]
    fn u8_to_enum() {
        let event_flag: EventFlag = EventFlag::Important;
        let enum_event_flag: EventFlag = EventFlag::try_from(0b00000010).unwrap();

        assert_eq!(event_flag, enum_event_flag);
    }
}