// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::ResponseParameters;

impl ResponseParameters {
    /// This function creates an empty struct for the object ResponseParameters.
    pub fn new() -> Self {
        Self {
            migrate_to_chat_id: None,
            retry_after: None,
        }
    }
}
impl Default for ResponseParameters {
    fn default() -> Self {
        Self::new()
    }
}
