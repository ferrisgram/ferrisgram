// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::InlineQueryResultCachedAudio;

impl InlineQueryResultCachedAudio {
    /// This function creates an empty struct for the object InlineQueryResultCachedAudio.
    pub fn new() -> Self {
        Self {
            r#type: "".to_string(),
            id: "".to_string(),
            audio_file_id: "".to_string(),
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}
impl Default for InlineQueryResultCachedAudio {
    fn default() -> Self {
        Self::new()
    }
}
