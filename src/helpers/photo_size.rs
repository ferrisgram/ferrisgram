// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments, clippy::new_without_default)]
use crate::types::PhotoSize;

impl PhotoSize {
    /// This function creates an empty struct for the object PhotoSize.
    pub fn new(file_id: String, file_unique_id: String, width: i64, height: i64) -> Self {
        Self {
            file_id,
            file_unique_id,
            width,
            height,
            file_size: None,
        }
    }
}
