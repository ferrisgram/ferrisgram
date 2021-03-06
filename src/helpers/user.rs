// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::User;

impl User {
    /// This function creates an empty struct for the object User.
    pub fn new() -> Self {
        Self {
            id: 0,
            is_bot: false,
            first_name: "".to_string(),
            last_name: None,
            username: None,
            language_code: None,
            is_premium: None,
            added_to_attachment_menu: None,
            can_join_groups: None,
            can_read_all_group_messages: None,
            supports_inline_queries: None,
        }
    }
}
impl Default for User {
    fn default() -> Self {
        Self::new()
    }
}
