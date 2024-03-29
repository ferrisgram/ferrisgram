// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::error::Result;
use crate::types::StickerSet;
use crate::Bot;

impl Bot {
    /// Use this method to get a sticker set. On success, a StickerSet object is returned.
    /// <https://core.telegram.org/bots/api#getstickerset>
    pub fn get_sticker_set(&self, name: String) -> GetStickerSetBuilder {
        GetStickerSetBuilder::new(self, name)
    }
}

#[derive(Serialize)]
pub struct GetStickerSetBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Name of the sticker set
    pub name: String,
}

impl<'a> GetStickerSetBuilder<'a> {
    pub fn new(bot: &'a Bot, name: String) -> Self {
        Self { bot, name }
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub async fn send(self) -> Result<StickerSet> {
        let form = serde_json::to_value(&self)?;
        self.bot.get("getStickerSet", Some(&form)).await
    }
}
