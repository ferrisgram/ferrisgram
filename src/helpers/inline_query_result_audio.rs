// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::InlineQueryResultAudio;

impl InlineQueryResultAudio {
    /// This function creates an empty struct for the object InlineQueryResultAudio.
    pub fn new() -> Self {
        Self {
            r#type: "".to_string(),
            id: "".to_string(),
            audio_url: "".to_string(),
            title: "".to_string(),
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
impl Default for InlineQueryResultAudio {
    fn default() -> Self {
        Self::new()
    }
}
