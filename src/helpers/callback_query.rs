// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::CallbackQuery;
use crate::types::User;

impl CallbackQuery {
    /// This function creates an empty struct for the object CallbackQuery.
    pub fn new() -> Self {
        Self {
            id: "".to_string(),
            from: User::new(),
            message: None,
            inline_message_id: None,
            chat_instance: "".to_string(),
            data: None,
            game_short_name: None,
        }
    }
}