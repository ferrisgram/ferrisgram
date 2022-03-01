use crate::methods::SendMessageBuilder;
use crate::types::{InlineKeyboardButton, Message};
use crate::Bot;

impl Message {
    /// This is a helper method to easily build the SendMessageBuilder with current message's chat id and message id.
    pub fn reply<'a>(&self, bot: &'a Bot, text: &'a str) -> SendMessageBuilder<'a> {
        SendMessageBuilder::new(bot, self.chat.id, text.to_string())
            .reply_to_message_id(self.message_id)
    }
    /// This is a helper method to get hyperlink of a message.
    /// It will return an empty string in case of private and group chat type.
    pub fn get_link(&self) -> String {
        if self.chat.r#type == "private" || self.chat.r#type == "group" {
            return String::new();
        }
        if self.chat.username.is_some() {
            return format!(
                "https://t.me/{}/{}",
                self.chat.username.as_ref().unwrap(),
                self.message_id
            );
        }
        format!(
            "https://t.me/c/{}/{}",
            self.chat.id.to_string().trim_start_matches("-100"),
            self.message_id
        )
    }
}

impl InlineKeyboardButton {
    pub fn callback_button(text: &str, callback_data: &str) -> Self {
        Self {
            text: text.to_string(),
            callback_data: Some(callback_data.to_string()),
            url: None,
            login_url: None,
            switch_inline_query: None,
            switch_inline_query_current_chat: None,
            callback_game: None,
            pay: None,
        }
    }
    pub fn url_button(text: &str, url: &str) -> Self {
        Self {
            text: text.to_string(),
            url: Some(url.to_string()),
            login_url: None,
            callback_data: None,
            switch_inline_query: None,
            switch_inline_query_current_chat: None,
            callback_game: None,
            pay: None,
        }
    }
}

pub trait StringPatch {
    fn get_args(&self) -> Vec<&str>
    where
        Self: Sized;
}

impl StringPatch for String {
    fn get_args(&self) -> Vec<&str> {
        self.split_whitespace().collect()
    }
}
