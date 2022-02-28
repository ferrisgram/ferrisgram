// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::error::Result;
use crate::types::InputFile;
use crate::Bot;

impl Bot {
    /// Use this method to set a new profile photo for the chat. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns True on success.
    /// <https://core.telegram.org/bots/api#setchatphoto>
    pub fn set_chat_photo(&self, chat_id: i64, photo: InputFile) -> SetChatPhotoBuilder {
        SetChatPhotoBuilder::new(self, chat_id, photo)
    }
}

#[derive(Serialize)]
pub struct SetChatPhotoBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
    /// New chat photo, uploaded using multipart/form-data
    pub photo: InputFile,
}

impl<'a> SetChatPhotoBuilder<'a> {
    pub fn new(bot: &'a Bot, chat_id: i64, photo: InputFile) -> Self {
        Self {
            bot,
            chat_id,
            photo,
        }
    }

    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }

    pub fn photo(mut self, photo: InputFile) -> Self {
        self.photo = photo;
        self
    }

    pub async fn send(self) -> Result<bool> {
        let form = serde_json::to_value(&self)?;
        self.bot.get::<bool>("setChatPhoto", Some(&form)).await
    }
}
