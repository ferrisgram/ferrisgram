// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::error::Result;
use crate::Bot;

impl Bot {
    /// Use this method to set a custom title for an administrator in a supergroup promoted by the bot. Returns True on success.
    /// <https://core.telegram.org/bots/api#setchatadministratorcustomtitle>
    pub fn set_chat_administrator_custom_title(
        &self,
        chat_id: i64,
        user_id: i64,
        custom_title: String,
    ) -> SetChatAdministratorCustomTitleBuilder {
        SetChatAdministratorCustomTitleBuilder::new(self, chat_id, user_id, custom_title)
    }
}

#[derive(Serialize)]
pub struct SetChatAdministratorCustomTitleBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: i64,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// New custom title for the administrator; 0-16 characters, emoji are not allowed
    pub custom_title: String,
}

impl<'a> SetChatAdministratorCustomTitleBuilder<'a> {
    pub fn new(bot: &'a Bot, chat_id: i64, user_id: i64, custom_title: String) -> Self {
        Self {
            bot,
            chat_id,
            user_id,
            custom_title,
        }
    }

    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }

    pub fn user_id(mut self, user_id: i64) -> Self {
        self.user_id = user_id;
        self
    }

    pub fn custom_title(mut self, custom_title: String) -> Self {
        self.custom_title = custom_title;
        self
    }

    pub async fn send(self) -> Result<bool> {
        let form = serde_json::to_value(&self)?;
        self.bot
            .get("setChatAdministratorCustomTitle", Some(&form))
            .await
    }
}
