// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::InlineKeyboardButton;
use serde::{Deserialize, Serialize};

/// This object represents an inline keyboard that appears right next to the message it belongs to.
/// <https://core.telegram.org/bots/api#inlinekeyboardmarkup>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InlineKeyboardMarkup {
    /// Array of button rows, each represented by an Array of InlineKeyboardButton objects
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}
