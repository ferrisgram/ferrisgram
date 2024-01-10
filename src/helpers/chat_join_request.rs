// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments, clippy::new_without_default)]
use crate::types::Chat;
use crate::types::ChatJoinRequest;
use crate::types::User;

impl ChatJoinRequest {
    /// This function creates an empty struct for the object ChatJoinRequest.
    pub fn new(chat: Chat, from: User, user_chat_id: i64, date: i64) -> Self {
        Self {
            chat,
            from,
            user_chat_id,
            date,
            bio: None,
            invite_link: None,
        }
    }
}
