// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::InlineQueryResultCachedVideo;

impl InlineQueryResultCachedVideo {
    /// This function creates an empty struct for the object InlineQueryResultCachedVideo.
    pub fn new() -> Self {
        Self {
            id: "".to_string(),
            video_file_id: "".to_string(),
            title: "".to_string(),
            description: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}
impl Default for InlineQueryResultCachedVideo {
    fn default() -> Self {
        Self::new()
    }
}
