// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments, clippy::new_without_default)]
use crate::types::InlineQueryResultCachedDocument;

impl InlineQueryResultCachedDocument {
    /// This function creates an empty struct for the object InlineQueryResultCachedDocument.
    pub fn new(id: String, title: String, document_file_id: String) -> Self {
        Self {
            id,
            title,
            document_file_id,
            description: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            reply_markup: None,
            input_message_content: None,
        }
    }
}
