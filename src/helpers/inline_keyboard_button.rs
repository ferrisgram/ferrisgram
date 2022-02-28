// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::InlineKeyboardButton;

impl InlineKeyboardButton {
    /// This function creates an empty struct for the object InlineKeyboardButton.
    pub fn new() -> Self {
        Self {
            text: "".to_string(),
            url: None,
            login_url: None,
            callback_data: None,
            switch_inline_query: None,
            switch_inline_query_current_chat: None,
            callback_game: None,
            pay: None,
        }
    }
}
impl Default for InlineKeyboardButton {
    fn default() -> Self {
        Self::new()
    }
}