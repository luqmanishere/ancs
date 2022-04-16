#[cfg(test)]
mod command_id {
    use ancs::attributes::command::CommandID;

    #[test]
    fn enum_to_u8() {
        let event_id: CommandID = CommandID::GetNotificationAttributes;
        let u8_event_id: u8 = event_id.into();

        assert_eq!(u8::MIN, u8_event_id)
    }

    #[test]
    fn u8_to_enum() {
        let event_id: CommandID = CommandID::GetNotificationAttributes;
        let enum_event_id: CommandID = CommandID::try_from(u8::MIN).unwrap();

        assert_eq!(event_id, enum_event_id);
    }
}
