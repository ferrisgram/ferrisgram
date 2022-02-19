// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::User;
use serde::{Deserialize, Serialize};


/// Represents a chat member that isn't currently a member of the chat, but may join it themselves.
/// <https://core.telegram.org/bots/api#chatmemberleft>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMemberLeft {
    /// The member's status in the chat, always "left"
    pub status: String,
    /// Information about the user
    pub user: User,
}