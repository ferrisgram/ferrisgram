// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::InlineQueryResultCachedGif;

impl InlineQueryResultCachedGif {
    /// This function creates an empty struct for the object InlineQueryResultCachedGif.
    pub fn new(id: String, gif_file_id: String) -> Self {
        Self {
            id,
            gif_file_id,
            title: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}
