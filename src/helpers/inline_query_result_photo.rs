// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::InlineQueryResultPhoto;

impl InlineQueryResultPhoto {
    /// This function creates an empty struct for the object InlineQueryResultPhoto.
    pub fn new() -> Self {
        Self {
            id: "".to_string(),
            photo_url: "".to_string(),
            thumb_url: "".to_string(),
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
impl Default for InlineQueryResultPhoto {
    fn default() -> Self {
        Self::new()
    }
}
