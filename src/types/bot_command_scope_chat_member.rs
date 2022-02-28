// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use serde::{Deserialize, Serialize};

/// Represents the scope of bot commands, covering a specific member of a group or supergroup chat.
/// <https://core.telegram.org/bots/api#botcommandscopechatmember>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BotCommandScopeChatMember {
    /// Scope type, must be chat_member
    pub r#type: String,
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: i64,
    /// Unique identifier of the target user
    pub user_id: i64,
}
