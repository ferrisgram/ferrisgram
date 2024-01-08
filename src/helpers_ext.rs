use crate::methods::SendMessageBuilder;
use crate::types::{
    InaccessibleMessage, InlineKeyboardButton, MaybeInaccessibleMessage, Message, ReplyParameters,
};
use crate::Bot;
use serde::{Deserialize, Deserializer};

impl Message {
    /// This is a helper method to easily build the SendMessageBuilder with current message's chat id and message id.
    pub fn reply<'a>(&self, bot: &'a Bot, text: &'a str) -> SendMessageBuilder<'a> {
        SendMessageBuilder::new(bot, self.chat.id, text.to_string())
            .reply_parameters(ReplyParameters::new(self.message_id))
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
            switch_inline_query_chosen_chat: None,
            callback_game: None,
            pay: None,
            web_app: None,
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
            switch_inline_query_chosen_chat: None,
            callback_game: None,
            pay: None,
            web_app: None,
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

impl<'de> Deserialize<'de> for MaybeInaccessibleMessage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;
        let a = serde_json::ser::to_string(&value).unwrap();
        if value["date"].as_i64() == Some(0) {
            return Ok(MaybeInaccessibleMessage::InaccessibleMessage(
                serde_json::de::from_str::<InaccessibleMessage>(a.as_str()).unwrap(),
            ));
        }
        return Ok(MaybeInaccessibleMessage::Message(
            serde_json::de::from_str::<Message>(a.as_str()).unwrap(),
        ));
    }
}
