// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::InlineQueryResultLocation;

impl InlineQueryResultLocation {
    /// This function creates an empty struct for the object InlineQueryResultLocation.
    pub fn new(id: String, latitude: f64, longitude: f64, title: String) -> Self {
        Self {
            id,
            latitude,
            longitude,
            title,
            horizontal_accuracy: None,
            live_period: None,
            heading: None,
            proximity_alert_radius: None,
            reply_markup: None,
            input_message_content: None,
            thumbnail_url: None,
            thumbnail_width: None,
            thumbnail_height: None,
        }
    }
}
