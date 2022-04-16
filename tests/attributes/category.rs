#[cfg(test)]
mod category_id {
    use ancs::attributes::category::CategoryID;

    #[test]
    fn enum_to_u8() {
        let event_id: CategoryID = CategoryID::Other;
        let u8_event_id: u8 = event_id.into();

        assert_eq!(u8::MIN, u8_event_id)
    }

    #[test]
    fn u8_to_enum() {
        let event_id: CategoryID = CategoryID::Other;
        let enum_event_id: CategoryID = CategoryID::try_from(u8::MIN).unwrap();

        assert_eq!(event_id, enum_event_id);
    }
}
