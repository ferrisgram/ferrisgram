// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use serde::Serialize;

use crate::Bot;
use crate::error::Result;
use crate::types::{InputFile, MaskPosition};

impl Bot {
    /// Use this method to create a new sticker set owned by a user. The bot will be able to edit the sticker set thus created. You must use exactly one of the fields png_sticker, tgs_sticker, or webm_sticker. Returns True on success.
    /// <https://core.telegram.org/bots/api#createnewstickerset>
    pub fn create_new_sticker_set(&self, user_id: i64, name: String, title: String, emojis: String) -> CreateNewStickerSetBuilder {
        CreateNewStickerSetBuilder::new(self, user_id, name, title, emojis)
    }
}

#[derive(Serialize)]
pub struct CreateNewStickerSetBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// User identifier of created sticker set owner
    pub user_id: i64,
    /// Short name of sticker set, to be used in t.me/addstickers/ URLs (e.g., animals). Can contain only english letters, digits and underscores. Must begin with a letter, can't contain consecutive underscores and must end in "_by_<bot username>". <bot_username> is case insensitive. 1-64 characters.
    pub name: String,
    /// Sticker set title, 1-64 characters
    pub title: String,
    /// PNG image with the sticker, must be up to 512 kilobytes in size, dimensions must not exceed 512px, and either width or height must be exactly 512px. Pass a file_id as a String to send a file that already exists on the Telegram servers, pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. More info on Sending Files: https://core.telegram.org/bots/api#sending-files
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
    /// Pass True, if a set of mask stickers should be created
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains_masks: Option<bool>,
    /// A JSON-serialized object for position where the mask should be placed on faces
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
}


impl <'a> CreateNewStickerSetBuilder<'a> {
    pub fn new(bot: &'a Bot, user_id: i64, name: String, title: String, emojis: String) -> Self {
        Self{
            bot,
            user_id,
            name,
            title,
            png_sticker: None,
            tgs_sticker: None,
            webm_sticker: None,
            emojis,
            contains_masks: None,
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
                
    pub fn title(mut self, title: String) -> Self {
        self.title = title;
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
                
    pub fn contains_masks(mut self, contains_masks: bool) -> Self {
        self.contains_masks = Some(contains_masks);
        self
    }
                
    pub fn mask_position(mut self, mask_position: MaskPosition) -> Self {
        self.mask_position = Some(mask_position);
        self
    }
                
    pub async fn send(self) -> Result<bool> {
        let form = serde_json::to_value(&self)?;
        self.bot.get::<bool>("createNewStickerSet", Some(&form)).await
    }

}