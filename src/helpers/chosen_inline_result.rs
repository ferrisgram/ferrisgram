// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments, clippy::new_without_default)]
use crate::types::ChosenInlineResult;
use crate::types::User;

impl ChosenInlineResult {
    /// This function creates an empty struct for the object ChosenInlineResult.
    pub fn new(result_id: String, from: User, query: String) -> Self {
        Self {
            result_id,
            from,
            location: None,
            inline_message_id: None,
            query,
        }
    }
}
