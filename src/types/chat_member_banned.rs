// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::User;
use serde::{Deserialize, Serialize};

/// Represents a chat member that was banned in the chat and can't return to the chat or view chat messages.
/// <https://core.telegram.org/bots/api#chatmemberbanned>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMemberBanned {
    /// The member's status in the chat, always "kicked"
    pub status: String,
    /// Information about the user
    pub user: User,
    /// Date when restrictions will be lifted for this user; Unix time. If 0, then the user is banned forever
    pub until_date: i64,
}
