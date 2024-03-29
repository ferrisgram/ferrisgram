// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::error::Result;
use crate::Bot;

impl Bot {
    /// Use this method to change the description of a group, a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success.
    /// <https://core.telegram.org/bots/api#setchatdescription>
    pub fn set_chat_description(&self, chat_id: i64) -> SetChatDescriptionBuilder {
        SetChatDescriptionBuilder::new(self, chat_id)
    }
}

#[derive(Serialize)]
pub struct SetChatDescriptionBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
    /// New chat description, 0-255 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl<'a> SetChatDescriptionBuilder<'a> {
    pub fn new(bot: &'a Bot, chat_id: i64) -> Self {
        Self {
            bot,
            chat_id,
            description: None,
        }
    }

    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub async fn send(self) -> Result<bool> {
        let form = serde_json::to_value(&self)?;
        self.bot.get("setChatDescription", Some(&form)).await
    }
}
