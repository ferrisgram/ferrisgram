// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::ChosenInlineResult;
use crate::types::User;

impl ChosenInlineResult {
    /// This function creates an empty struct for the object ChosenInlineResult.
    pub fn new() -> Self {
        Self {
            result_id: "".to_string(),
            from: User::new(),
            location: None,
            inline_message_id: None,
            query: "".to_string(),
        }
    }
}
impl Default for ChosenInlineResult {
    fn default() -> Self {
        Self::new()
    }
}
