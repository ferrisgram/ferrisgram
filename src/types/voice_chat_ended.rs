// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use serde::{Deserialize, Serialize};


/// This object represents a service message about a voice chat ended in the chat.
/// <https://core.telegram.org/bots/api#voicechatended>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VoiceChatEnded {
    /// Voice chat duration in seconds
    pub duration: i64,
}