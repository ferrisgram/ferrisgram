// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::InlineQueryResultVenue;

impl InlineQueryResultVenue {
    /// This function creates an empty struct for the object InlineQueryResultVenue.
    pub fn new() -> Self {
        Self {
            r#type: "".to_string(),
            id: "".to_string(),
            latitude: 0.0,
            longitude: 0.0,
            title: "".to_string(),
            address: "".to_string(),
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
            reply_markup: None,
            input_message_content: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        }
    }
}