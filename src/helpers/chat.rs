// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::Chat;

impl Chat {
    /// This function creates an empty struct for the object Chat.
    pub fn new() -> Self {
        Self {
            id: 0,
            r#type: "".to_string(),
            title: None,
            username: None,
            first_name: None,
            last_name: None,
            photo: None,
            bio: None,
            has_private_forwards: None,
            join_to_send_messages: None,
            join_by_request: None,
            description: None,
            invite_link: None,
            pinned_message: None,
            permissions: None,
            slow_mode_delay: None,
            message_auto_delete_time: None,
            has_protected_content: None,
            sticker_set_name: None,
            can_set_sticker_set: None,
            linked_chat_id: None,
            location: None,
        }
    }
}
impl Default for Chat {
    fn default() -> Self {
        Self::new()
    }
}
