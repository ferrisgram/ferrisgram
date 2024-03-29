// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::Chat;
use serde::{Deserialize, Serialize};

/// The message was originally sent on behalf of a chat to a group chat.
/// <https://core.telegram.org/bots/api#messageoriginchat>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageOriginChat {
    /// Date the message was sent originally in Unix time
    pub date: i64,
    /// Chat that sent the message originally
    pub sender_chat: Chat,
    /// Optional. For messages originally sent by an anonymous chat administrator, original message author signature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_signature: Option<String>,
}
