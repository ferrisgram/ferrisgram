// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::error::Result;
use crate::types::InputSticker;
use crate::Bot;

impl Bot {
    /// Use this method to add a new sticker to a set created by the bot. The format of the added sticker must match the format of the other stickers in the set. Emoji sticker sets can have up to 200 stickers. Animated and video sticker sets can have up to 50 stickers. Static sticker sets can have up to 120 stickers. Returns True on success.
    /// <https://core.telegram.org/bots/api#addstickertoset>
    pub fn add_sticker_to_set(
        &self,
        user_id: i64,
        name: String,
        sticker: InputSticker,
    ) -> AddStickerToSetBuilder {
        AddStickerToSetBuilder::new(self, user_id, name, sticker)
    }
}

#[derive(Serialize)]
pub struct AddStickerToSetBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// User identifier of sticker set owner
    pub user_id: i64,
    /// Sticker set name
    pub name: String,
    /// A JSON-serialized object with information about the added sticker. If exactly the same sticker had already been added to the set, then the set isn't changed.
    pub sticker: InputSticker,
}

impl<'a> AddStickerToSetBuilder<'a> {
    pub fn new(bot: &'a Bot, user_id: i64, name: String, sticker: InputSticker) -> Self {
        Self {
            bot,
            user_id,
            name,
            sticker,
        }
    }

    pub fn user_id(mut self, user_id: i64) -> Self {
        self.user_id = user_id;
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn sticker(mut self, sticker: InputSticker) -> Self {
        self.sticker = sticker;
        self
    }

    pub async fn send(self) -> Result<bool> {
        let form = serde_json::to_value(&self)?;
        self.bot.get("addStickerToSet", Some(&form)).await
    }
}
