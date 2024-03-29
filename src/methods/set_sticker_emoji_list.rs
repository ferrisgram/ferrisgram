// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::error::Result;
use crate::Bot;

impl Bot {
    /// Use this method to change the list of emoji assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns True on success.
    /// <https://core.telegram.org/bots/api#setstickeremojilist>
    pub fn set_sticker_emoji_list(
        &self,
        sticker: String,
        emoji_list: Vec<String>,
    ) -> SetStickerEmojiListBuilder {
        SetStickerEmojiListBuilder::new(self, sticker, emoji_list)
    }
}

#[derive(Serialize)]
pub struct SetStickerEmojiListBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// File identifier of the sticker
    pub sticker: String,
    /// A JSON-serialized list of 1-20 emoji associated with the sticker
    pub emoji_list: Vec<String>,
}

impl<'a> SetStickerEmojiListBuilder<'a> {
    pub fn new(bot: &'a Bot, sticker: String, emoji_list: Vec<String>) -> Self {
        Self {
            bot,
            sticker,
            emoji_list,
        }
    }

    pub fn sticker(mut self, sticker: String) -> Self {
        self.sticker = sticker;
        self
    }

    pub fn emoji_list(mut self, emoji_list: Vec<String>) -> Self {
        self.emoji_list = emoji_list;
        self
    }

    pub async fn send(self) -> Result<bool> {
        let form = serde_json::to_value(&self)?;
        self.bot.get("setStickerEmojiList", Some(&form)).await
    }
}
