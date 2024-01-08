// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::{LinkPreviewOptions, MessageEntity};
use serde::{Deserialize, Serialize};

/// Represents the content of a text message to be sent as the result of an inline query.
/// <https://core.telegram.org/bots/api#inputtextmessagecontent>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InputTextMessageContent {
    /// Optional. Mode for parsing entities in the message text. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// Optional. List of special entities that appear in message text, which can be specified instead of parse_mode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    /// Optional. Link preview generation options for the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<LinkPreviewOptions>,
}
