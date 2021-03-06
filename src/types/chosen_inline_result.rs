// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::{Location, User};
use serde::{Deserialize, Serialize};

/// Represents a result of an inline query that was chosen by the user and sent to their chat partner.
/// Note: It is necessary to enable inline feedback via @BotFather in order to receive these objects in updates.
/// <https://core.telegram.org/bots/api#choseninlineresult>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChosenInlineResult {
    /// The unique identifier for the result that was chosen
    pub result_id: String,
    /// The user that chose the result
    pub from: User,
    /// Optional. Sender location, only for bots that require user location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /// Optional. Identifier of the sent inline message. Available only if there is an inline keyboard attached to the message. Will be also received in callback queries and can be used to edit the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// The query that was used to obtain the result
    pub query: String,
}
