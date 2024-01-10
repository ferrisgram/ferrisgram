// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::error::Result;
use crate::types::MaskPosition;
use crate::Bot;

impl Bot {
    /// Use this method to change the mask position of a mask sticker. The sticker must belong to a sticker set that was created by the bot. Returns True on success.
    /// <https://core.telegram.org/bots/api#setstickermaskposition>
    pub fn set_sticker_mask_position(&self, sticker: String) -> SetStickerMaskPositionBuilder {
        SetStickerMaskPositionBuilder::new(self, sticker)
    }
}

#[derive(Serialize)]
pub struct SetStickerMaskPositionBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// File identifier of the sticker
    pub sticker: String,
    /// A JSON-serialized object with the position where the mask should be placed on faces. Omit the parameter to remove the mask position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
}

impl<'a> SetStickerMaskPositionBuilder<'a> {
    pub fn new(bot: &'a Bot, sticker: String) -> Self {
        Self {
            bot,
            sticker,
            mask_position: None,
        }
    }

    pub fn sticker(mut self, sticker: String) -> Self {
        self.sticker = sticker;
        self
    }

    pub fn mask_position(mut self, mask_position: MaskPosition) -> Self {
        self.mask_position = Some(mask_position);
        self
    }

    pub async fn send(self) -> Result<bool> {
        let form = serde_json::to_value(&self)?;
        self.bot.get("setStickerMaskPosition", Some(&form)).await
    }
}
