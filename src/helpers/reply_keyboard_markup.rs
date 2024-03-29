// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments, clippy::new_without_default)]
use crate::types::KeyboardButton;
use crate::types::ReplyKeyboardMarkup;

impl ReplyKeyboardMarkup {
    /// This function creates an empty struct for the object ReplyKeyboardMarkup.
    pub fn new(keyboard: Vec<Vec<KeyboardButton>>) -> Self {
        Self {
            keyboard,
            is_persistent: None,
            resize_keyboard: None,
            one_time_keyboard: None,
            input_field_placeholder: None,
            selective: None,
        }
    }
}
