// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::InlineQueryResultArticle;
use crate::types::InputMessageContent;

impl InlineQueryResultArticle {
    /// This function creates an empty struct for the object InlineQueryResultArticle.
    pub fn new() -> Self {
        Self {
            id: "".to_string(),
            title: "".to_string(),
            input_message_content: InputMessageContent::new(),
            reply_markup: None,
            url: None,
            hide_url: None,
            description: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        }
    }
}
impl Default for InlineQueryResultArticle {
    fn default() -> Self {
        Self::new()
    }
}
