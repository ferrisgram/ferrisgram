// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::Bot;
use crate::error::Result;
use crate::types::InputFile;

impl Bot {
    /// Use this method to set the thumbnail of a sticker set. Animated thumbnails can be set for animated sticker sets only. Video thumbnails can be set only for video sticker sets only. Returns True on success.
    /// <https://core.telegram.org/bots/api#setstickersetthumb>
    pub fn set_sticker_set_thumb(&self, name: String, user_id: i64) -> SetStickerSetThumbBuilder {
        SetStickerSetThumbBuilder::new(self, name, user_id)
    }
}

#[derive(Serialize)]
pub struct SetStickerSetThumbBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Sticker set name
    pub name: String,
    /// User identifier of the sticker set owner
    pub user_id: i64,
    /// A PNG image with the thumbnail, must be up to 128 kilobytes in size and have width and height exactly 100px, or a TGS animation with the thumbnail up to 32 kilobytes in size; see https://core.telegram.org/stickers#animated-sticker-requirements for animated sticker technical requirements, or a WEBM video with the thumbnail up to 32 kilobytes in size; see https://core.telegram.org/stickers#video-sticker-requirements for video sticker technical requirements. Pass a file_id as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. More information on Sending Files: https://core.telegram.org/bots/api#sending-files. Animated sticker set thumbnails can't be uploaded via HTTP URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<String>,
}


impl <'a> SetStickerSetThumbBuilder<'a> {
    pub fn new(bot: &'a Bot, name: String, user_id: i64) -> Self {
        Self{
            bot,
            name,
            user_id,
            thumb: None,
        }
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
                
    pub fn user_id(mut self, user_id: i64) -> Self {
        self.user_id = user_id;
        self
    }
                
    pub fn thumb(mut self, thumb: String) -> Self {
        self.thumb = Some(thumb);
        self
    }
                
    pub async fn send(self) -> Result<bool> {
        let form = serde_json::to_value(&self)?;
        self.bot.get::<bool>("setStickerSetThumb", Some(&form)).await
    }

}