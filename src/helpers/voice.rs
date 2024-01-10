// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments, clippy::new_without_default)]
use crate::types::Voice;

impl Voice {
    /// This function creates an empty struct for the object Voice.
    pub fn new(file_id: String, file_unique_id: String, duration: i64) -> Self {
        Self {
            file_id,
            file_unique_id,
            duration,
            mime_type: None,
            file_size: None,
        }
    }
}
