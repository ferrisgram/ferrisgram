// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::InlineQueryResultCachedPhoto;

impl InlineQueryResultCachedPhoto {
    /// This function creates an empty struct for the object InlineQueryResultCachedPhoto.
    pub fn new(r#type: String, id: String, photo_file_id: String) -> Self {
        Self {
            r#type,
            id,
            photo_file_id,
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
