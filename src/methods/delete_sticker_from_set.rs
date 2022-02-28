// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]use serde::Serialize;

use crate::Bot;
use crate::error::Result;

impl Bot {
    /// Use this method to delete a sticker from a set created by the bot. Returns True on success.
    /// <https://core.telegram.org/bots/api#deletestickerfromset>
    pub fn delete_sticker_from_set(&self, sticker: String) -> DeleteStickerFromSetBuilder {
        DeleteStickerFromSetBuilder::new(self, sticker)
    }
}

#[derive(Serialize)]
pub struct DeleteStickerFromSetBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// File identifier of the sticker
    pub sticker: String,
}


impl <'a> DeleteStickerFromSetBuilder<'a> {
    pub fn new(bot: &'a Bot, sticker: String) -> Self {
        Self{
            bot,
            sticker,
        }
    }

    pub fn sticker(mut self, sticker: String) -> Self {
        self.sticker = sticker;
        self
    }
                
    pub async fn send(self) -> Result<bool> {
        let form = serde_json::to_value(&self)?;
        self.bot.get::<bool>("deleteStickerFromSet", Some(&form)).await
    }

}