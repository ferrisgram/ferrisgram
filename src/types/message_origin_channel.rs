// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::Chat;
use serde::{Deserialize, Serialize};

/// The message was originally sent to a channel chat.
/// <https://core.telegram.org/bots/api#messageoriginchannel>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageOriginChannel {
    /// Date the message was sent originally in Unix time
    pub date: i64,
    /// Channel chat to which the message was originally sent
    pub chat: Chat,
    /// Unique message identifier inside the chat
    pub message_id: i64,
    /// Optional. Signature of the original post author
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_signature: Option<String>,
}
