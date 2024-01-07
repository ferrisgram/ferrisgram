// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use serde::{Deserialize, Serialize};

/// Represents a menu button, which opens the bot's list of commands.
/// <https://core.telegram.org/bots/api#menubuttoncommands>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuButtonCommands {
    /// Type of the button, must be commands
    pub r#type: String,
}
