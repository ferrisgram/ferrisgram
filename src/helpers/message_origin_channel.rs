// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments, clippy::new_without_default)]
use crate::types::Chat;
use crate::types::MessageOriginChannel;

impl MessageOriginChannel {
    /// This function creates an empty struct for the object MessageOriginChannel.
    pub fn new(date: i64, chat: Chat, message_id: i64) -> Self {
        Self {
            date,
            chat,
            message_id,
            author_signature: None,
        }
    }
}
