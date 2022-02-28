// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]use serde::Serialize;

use crate::Bot;
use crate::error::Result;
use crate::types::{InputFile, MessageEntity, InlineKeyboardMarkup};
use crate::types::Message;

impl Bot {
    /// Use this method to send general files. On success, the sent Message is returned. Bots can currently send files of any type of up to 50 MB in size, this limit may be changed in the future.
    /// <https://core.telegram.org/bots/api#senddocument>
    pub fn send_document(&self, chat_id: i64, document: InputFile) -> SendDocumentBuilder {
        SendDocumentBuilder::new(self, chat_id, document)
    }
}

#[derive(Serialize)]
pub struct SendDocumentBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
    /// File to send. Pass a file_id as String to send a file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a file from the Internet, or upload a new one using multipart/form-data. More info on Sending Files: https://core.telegram.org/bots/api#sending-files
    pub document: InputFile,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass "attach://<file_attach_name>" if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More info on Sending Files: https://core.telegram.org/bots/api#sending-files
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<InputFile>,
    /// Document caption (may also be used when resending documents by file_id), 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Mode for parsing entities in the document caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Disables automatic server-side content type detection for files uploaded using multipart/form-data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_content_type_detection: Option<bool>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i64>,
    /// Pass True, if the message should be sent even if the specified replied-to message is not found
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}


impl <'a> SendDocumentBuilder<'a> {
    pub fn new(bot: &'a Bot, chat_id: i64, document: InputFile) -> Self {
        Self{
            bot,
            chat_id,
            document,
            thumb: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            disable_content_type_detection: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }

    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
                
    pub fn document(mut self, document: InputFile) -> Self {
        self.document = document;
        self
    }
                
    pub fn thumb(mut self, thumb: InputFile) -> Self {
        self.thumb = Some(thumb);
        self
    }
                
    pub fn caption(mut self, caption: String) -> Self {
        self.caption = Some(caption);
        self
    }
                
    pub fn parse_mode(mut self, parse_mode: String) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }
                
    pub fn caption_entities(mut self, caption_entities: Vec<MessageEntity>) -> Self {
        self.caption_entities = Some(caption_entities);
        self
    }
                
    pub fn disable_content_type_detection(mut self, disable_content_type_detection: bool) -> Self {
        self.disable_content_type_detection = Some(disable_content_type_detection);
        self
    }
                
    pub fn disable_notification(mut self, disable_notification: bool) -> Self {
        self.disable_notification = Some(disable_notification);
        self
    }
                
    pub fn protect_content(mut self, protect_content: bool) -> Self {
        self.protect_content = Some(protect_content);
        self
    }
                
    pub fn reply_to_message_id(mut self, reply_to_message_id: i64) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id);
        self
    }
                
    pub fn allow_sending_without_reply(mut self, allow_sending_without_reply: bool) -> Self {
        self.allow_sending_without_reply = Some(allow_sending_without_reply);
        self
    }
                
    pub fn reply_markup(mut self, reply_markup: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(reply_markup);
        self
    }
                
    pub async fn send(self) -> Result<Message> {
        let form = serde_json::to_value(&self)?;
        self.bot.get::<Message>("sendDocument", Some(&form)).await
    }

}