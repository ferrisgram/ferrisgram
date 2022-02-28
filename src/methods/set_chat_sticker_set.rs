// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::error::Result;
use crate::Bot;

impl Bot {
    /// Use this method to set a new group sticker set for a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field can_set_sticker_set optionally returned in getChat requests to check if the bot can use this method. Returns True on success.
    /// <https://core.telegram.org/bots/api#setchatstickerset>
    pub fn set_chat_sticker_set(
        &self,
        chat_id: i64,
        sticker_set_name: String,
    ) -> SetChatStickerSetBuilder {
        SetChatStickerSetBuilder::new(self, chat_id, sticker_set_name)
    }
}

#[derive(Serialize)]
pub struct SetChatStickerSetBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: i64,
    /// Name of the sticker set to be set as the group sticker set
    pub sticker_set_name: String,
}

impl<'a> SetChatStickerSetBuilder<'a> {
    pub fn new(bot: &'a Bot, chat_id: i64, sticker_set_name: String) -> Self {
        Self {
            bot,
            chat_id,
            sticker_set_name,
        }
    }

    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }

    pub fn sticker_set_name(mut self, sticker_set_name: String) -> Self {
        self.sticker_set_name = sticker_set_name;
        self
    }

    pub async fn send(self) -> Result<bool> {
        let form = serde_json::to_value(&self)?;
        self.bot.get::<bool>("setChatStickerSet", Some(&form)).await
    }
}
