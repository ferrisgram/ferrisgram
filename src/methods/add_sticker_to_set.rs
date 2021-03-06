// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::error::Result;
use crate::types::{InputFile, MaskPosition};
use crate::Bot;

impl Bot {
    /// Use this method to add a new sticker to a set created by the bot. You must use exactly one of the fields png_sticker, tgs_sticker, or webm_sticker. Animated stickers can be added to animated sticker sets and only to them. Animated sticker sets can have up to 50 stickers. Static sticker sets can have up to 120 stickers. Returns True on success.
    /// <https://core.telegram.org/bots/api#addstickertoset>
    pub fn add_sticker_to_set(
        &self,
        user_id: i64,
        name: String,
        emojis: String,
    ) -> AddStickerToSetBuilder {
        AddStickerToSetBuilder::new(self, user_id, name, emojis)
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
    /// PNG image with the sticker, must be up to 512 kilobytes in size, dimensions must not exceed 512px, and either width or height must be exactly 512px. Pass a file_id as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. More information on Sending Files: https://core.telegram.org/bots/api#sending-files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub png_sticker: Option<InputFile>,
    /// TGS animation with the sticker, uploaded using multipart/form-data. See https://core.telegram.org/stickers#animated-sticker-requirements for technical requirements
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tgs_sticker: Option<InputFile>,
    /// WEBM video with the sticker, uploaded using multipart/form-data. See https://core.telegram.org/stickers#video-sticker-requirements for technical requirements
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webm_sticker: Option<InputFile>,
    /// One or more emoji corresponding to the sticker
    pub emojis: String,
    /// A JSON-serialized object for position where the mask should be placed on faces
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
}

impl<'a> AddStickerToSetBuilder<'a> {
    pub fn new(bot: &'a Bot, user_id: i64, name: String, emojis: String) -> Self {
        Self {
            bot,
            user_id,
            name,
            png_sticker: None,
            tgs_sticker: None,
            webm_sticker: None,
            emojis,
            mask_position: None,
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

    pub fn png_sticker(mut self, png_sticker: InputFile) -> Self {
        self.png_sticker = Some(png_sticker);
        self
    }

    pub fn tgs_sticker(mut self, tgs_sticker: InputFile) -> Self {
        self.tgs_sticker = Some(tgs_sticker);
        self
    }

    pub fn webm_sticker(mut self, webm_sticker: InputFile) -> Self {
        self.webm_sticker = Some(webm_sticker);
        self
    }

    pub fn emojis(mut self, emojis: String) -> Self {
        self.emojis = emojis;
        self
    }

    pub fn mask_position(mut self, mask_position: MaskPosition) -> Self {
        self.mask_position = Some(mask_position);
        self
    }

    pub async fn send(self) -> Result<bool> {
        let form = serde_json::to_value(&self)?;
        self.bot.get::<bool>("addStickerToSet", Some(&form)).await
    }
}
