// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::InlineKeyboardButton;

impl InlineKeyboardButton {
    /// This function creates an empty struct for the object InlineKeyboardButton.
    pub fn new(text: String) -> Self {
        Self {
            text,
            url: None,
            callback_data: None,
            web_app: None,
            login_url: None,
            switch_inline_query: None,
            switch_inline_query_current_chat: None,
            switch_inline_query_chosen_chat: None,
            callback_game: None,
            pay: None,
        }
    }
}
