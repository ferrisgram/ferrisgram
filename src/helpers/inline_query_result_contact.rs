// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::InlineQueryResultContact;

impl InlineQueryResultContact {
    /// This function creates an empty struct for the object InlineQueryResultContact.
    pub fn new() -> Self {
        Self {
            r#type: "".to_string(),
            id: "".to_string(),
            phone_number: "".to_string(),
            first_name: "".to_string(),
            last_name: None,
            vcard: None,
            reply_markup: None,
            input_message_content: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        }
    }
}