// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments, clippy::new_without_default)]
use crate::types::InlineQueryResultAudio;

impl InlineQueryResultAudio {
    /// This function creates an empty struct for the object InlineQueryResultAudio.
    pub fn new(id: String, audio_url: String, title: String) -> Self {
        Self {
            id,
            audio_url,
            title,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            performer: None,
            audio_duration: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}
