// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::InputMediaAudio;

impl InputMediaAudio {
    /// This function creates an empty struct for the object InputMediaAudio.
    pub fn new() -> Self {
        Self {
            r#type: "".to_string(),
            media: "".to_string(),
            thumb: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            duration: None,
            performer: None,
            title: None,
        }
    }
}
impl Default for InputMediaAudio {
    fn default() -> Self {
        Self::new()
    }
}
