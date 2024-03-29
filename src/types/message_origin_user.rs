// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::User;
use serde::{Deserialize, Serialize};

/// The message was originally sent by a known user.
/// <https://core.telegram.org/bots/api#messageoriginuser>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageOriginUser {
    /// Date the message was sent originally in Unix time
    pub date: i64,
    /// User that sent the message originally
    pub sender_user: User,
}
