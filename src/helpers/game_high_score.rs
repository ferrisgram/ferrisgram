// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::GameHighScore;
use crate::types::User;

impl GameHighScore {
    /// This function creates an empty struct for the object GameHighScore.
    pub fn new() -> Self {
        Self {
            position: 0,
            user: User::new(),
            score: 0,
        }
    }
}