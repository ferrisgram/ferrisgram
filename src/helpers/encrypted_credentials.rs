// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::EncryptedCredentials;

impl EncryptedCredentials {
    /// This function creates an empty struct for the object EncryptedCredentials.
    pub fn new() -> Self {
        Self {
            data: "".to_string(),
            hash: "".to_string(),
            secret: "".to_string(),
        }
    }
}
impl Default for EncryptedCredentials {
    fn default() -> Self {
        Self::new()
    }
}
