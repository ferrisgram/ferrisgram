// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments, clippy::new_without_default)]
use crate::types::PassportElementErrorFile;

impl PassportElementErrorFile {
    /// This function creates an empty struct for the object PassportElementErrorFile.
    pub fn new(r#type: String, file_hash: String, message: String) -> Self {
        Self {
            r#type,
            file_hash,
            message,
        }
    }
}
