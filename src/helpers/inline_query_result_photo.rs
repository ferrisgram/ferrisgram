// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::InlineQueryResultPhoto;

impl InlineQueryResultPhoto {
    /// This function creates an empty struct for the object InlineQueryResultPhoto.
    pub fn new(id: String, photo_url: String, thumbnail_url: String) -> Self {
        Self {
            id,
            photo_url,
            thumbnail_url,
            photo_width: None,
            photo_height: None,
            title: None,
            description: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}
