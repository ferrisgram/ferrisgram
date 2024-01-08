// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::InputVenueMessageContent;

impl InputVenueMessageContent {
    /// This function creates an empty struct for the object InputVenueMessageContent.
    pub fn new(longitude: f64, title: String, address: String) -> Self {
        Self {
            longitude,
            title,
            address,
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
        }
    }
}
