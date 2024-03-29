// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments, clippy::new_without_default)]
use crate::types::Sticker;

impl Sticker {
    /// This function creates an empty struct for the object Sticker.
    pub fn new(
        file_id: String,
        file_unique_id: String,
        r#type: String,
        width: i64,
        height: i64,
        is_animated: bool,
        is_video: bool,
    ) -> Self {
        Self {
            file_id,
            file_unique_id,
            r#type,
            width,
            height,
            is_animated,
            is_video,
            thumbnail: None,
            emoji: None,
            set_name: None,
            premium_animation: None,
            mask_position: None,
            custom_emoji_id: None,
            needs_repainting: None,
            file_size: None,
        }
    }
}
