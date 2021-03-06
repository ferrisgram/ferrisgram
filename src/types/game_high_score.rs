// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::User;
use serde::{Deserialize, Serialize};

/// This object represents one row of the high scores table for a game.
/// <https://core.telegram.org/bots/api#gamehighscore>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameHighScore {
    /// Position in high score table for the game
    pub position: i64,
    /// User
    pub user: User,
    /// Score
    pub score: i64,
}
