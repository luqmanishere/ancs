pub use crate::attributes::attribute::*;
pub use crate::attributes::command::*;

pub const CONTROL_POINT_UUID: &str = "69D1D8F3-45E1-49A8-9821-9BBDFDAAD9D9";

pub struct GetNotificationAttributesRequest {
    pub command_id: CommandID,
    pub notification_uuid: u32,
    pub attribute_ids: Vec<AttributeID>,
}

impl Into<Vec<u8>> for GetNotificationAttributesRequest {
    fn into(self) -> Vec<u8> {
        let id = self.command_id as u8;
        let notification_uuid: [u8; 4] = self.notification_uuid.to_le_bytes();

        // Note we default the max size for attributes that require it to max to simplify the
        // rest of this programs structures this may change in the future based on consumers
        // demands.
        let mut attribute_ids: Vec<u8> = Vec::new();

        self.attribute_ids.into_iter().for_each(|id| {
            match id {
                AttributeID::Title => {
                    let byte: u8 = id.into();
                    let length_bytes: [u8; 2] = 65535_u16.to_le_bytes();
                    attribute_ids.push(byte);
                    attribute_ids.extend(length_bytes);
                }
                AttributeID::Subtitle => {
                    let byte: u8 = id.into();
                    let length_bytes: [u8; 2] = 65535_u16.to_le_bytes();
                    attribute_ids.push(byte);
                    attribute_ids.extend(length_bytes);
                }
                AttributeID::Message => {
                    let byte: u8 = id.into();
                    let length_bytes: [u8; 2] = 65535_u16.to_le_bytes();
                    attribute_ids.push(byte);
                    attribute_ids.extend(length_bytes);
                }
                _ => {
                    let bytes: u8 = id.into();
                    attribute_ids.push(bytes);
                }
            };
        });

        let mut v: Vec<u8> = Vec::new();

        v.push(id);
        v.extend(notification_uuid);
        v.append(&mut attribute_ids);

        return v;
    }
}

pub struct GetAppAttributesRequest {
    command_id: CommandID,
    app_identifier: String,
    attribute_ids: Vec<AttributeID>,
}

impl From<GetAppAttributesRequest> for Vec<u8> {
    fn from(original: GetAppAttributesRequest) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();

        // Convert all attributes to bytes
        let command_id: u8 = original.command_id.into();
        let mut app_identifier: Vec<u8> = original.app_identifier.into_bytes();
        let mut attribute_ids: Vec<u8> = original
            .attribute_ids
            .into_iter()
            .map(|id| id.into())
            .collect();

        vec.push(command_id);
        vec.append(&mut app_identifier);
        vec.append(&mut attribute_ids);

        return vec;
    }
}
