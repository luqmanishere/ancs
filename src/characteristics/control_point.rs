pub use crate::attributes::attribute::*;
pub use crate::attributes::command::*;

struct GetNotificationAttributesRequest {
    command_id: CommandID,
    notification_uuid: u32,
    attribute_ids: Vec<AttributeID>,
}

impl Into<Vec<u8>> for GetNotificationAttributesRequest {
    fn into(self) -> Vec<u8> {
        let id = self.command_id as u8;
        let notification_uuid: [u8; 4] = self.notification_uuid.to_le_bytes();
        let mut attribute_ids = self.attribute_ids.into_iter().map(|id| id as u8).collect();

        let mut v: Vec<u8> = Vec::new();
        
        v.push(id);
        v.extend(notification_uuid);
        v.append(&mut attribute_ids);

        return v;
    }
}

struct GetAppAttributesRequest {
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
        let mut attribute_ids: Vec<u8> = original.attribute_ids.into_iter().map(|id| id.into()).collect();

        vec.push(command_id);
        vec.append(&mut app_identifier);
        vec.append(&mut attribute_ids);

        return vec;
    }
}
