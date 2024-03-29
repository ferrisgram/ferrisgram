// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::error::Result;
use crate::types::ChatMember;
use crate::Bot;

impl Bot {
    /// Use this method to get a list of administrators in a chat, which aren't bots. Returns an Array of ChatMember objects.
    /// <https://core.telegram.org/bots/api#getchatadministrators>
    pub fn get_chat_administrators(&self, chat_id: i64) -> GetChatAdministratorsBuilder {
        GetChatAdministratorsBuilder::new(self, chat_id)
    }
}

#[derive(Serialize)]
pub struct GetChatAdministratorsBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    pub chat_id: i64,
}

impl<'a> GetChatAdministratorsBuilder<'a> {
    pub fn new(bot: &'a Bot, chat_id: i64) -> Self {
        Self { bot, chat_id }
    }

    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }

    pub async fn send(self) -> Result<Vec<ChatMember>> {
        let form = serde_json::to_value(&self)?;
        self.bot.get("getChatAdministrators", Some(&form)).await
    }
}
