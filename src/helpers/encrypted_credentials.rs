// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments, clippy::new_without_default)]
use crate::types::EncryptedCredentials;

impl EncryptedCredentials {
    /// This function creates an empty struct for the object EncryptedCredentials.
    pub fn new(data: String, hash: String, secret: String) -> Self {
        Self { data, hash, secret }
    }
}
