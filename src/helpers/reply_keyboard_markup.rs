// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::ReplyKeyboardMarkup;

impl ReplyKeyboardMarkup {
    /// This function creates an empty struct for the object ReplyKeyboardMarkup.
    pub fn new() -> Self {
        Self {
            keyboard: Vec::new(),
            resize_keyboard: None,
            one_time_keyboard: None,
            input_field_placeholder: None,
            selective: None,
        }
    }
}
impl Default for ReplyKeyboardMarkup {
    fn default() -> Self {
        Self::new()
    }
}