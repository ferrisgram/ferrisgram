// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::InlineQueryResultGif;

impl InlineQueryResultGif {
    /// This function creates an empty struct for the object InlineQueryResultGif.
    pub fn new(id: String, gif_url: String, thumbnail_url: String) -> Self {
        Self {
            id,
            gif_url,
            gif_width: None,
            gif_height: None,
            gif_duration: None,
            thumbnail_url,
            thumbnail_mime_type: None,
            title: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}
