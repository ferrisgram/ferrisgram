// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::InputMediaVideo;

impl InputMediaVideo {
    /// This function creates an empty struct for the object InputMediaVideo.
    pub fn new() -> Self {
        Self {
            r#type: "".to_string(),
            media: "".to_string(),
            thumb: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            width: None,
            height: None,
            duration: None,
            supports_streaming: None,
        }
    }
}
impl Default for InputMediaVideo {
    fn default() -> Self {
        Self::new()
    }
}