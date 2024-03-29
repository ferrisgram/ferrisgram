// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::error::Result;
use crate::types::Message;
use crate::Bot;

impl Bot {
    /// Use this method to set the score of the specified user in a game message. On success, if the message is not an inline message, the Message is returned, otherwise True is returned. Returns an error, if the new score is not greater than the user's current score in the chat and force is False.
    /// <https://core.telegram.org/bots/api#setgamescore>
    pub fn set_game_score(&self, user_id: i64, score: i64) -> SetGameScoreBuilder {
        SetGameScoreBuilder::new(self, user_id, score)
    }
}

#[derive(Serialize)]
pub struct SetGameScoreBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// User identifier
    pub user_id: i64,
    /// New score, must be non-negative
    pub score: i64,
    /// Pass True if the high score is allowed to decrease. This can be useful when fixing mistakes or banning cheaters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    /// Pass True if the game message should not be automatically edited to include the current scoreboard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_edit_message: Option<bool>,
    /// Required if inline_message_id is not specified. Unique identifier for the target chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<i64>,
    /// Required if inline_message_id is not specified. Identifier of the sent message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Required if chat_id and message_id are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}

impl<'a> SetGameScoreBuilder<'a> {
    pub fn new(bot: &'a Bot, user_id: i64, score: i64) -> Self {
        Self {
            bot,
            user_id,
            score,
            force: None,
            disable_edit_message: None,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
        }
    }

    pub fn user_id(mut self, user_id: i64) -> Self {
        self.user_id = user_id;
        self
    }

    pub fn score(mut self, score: i64) -> Self {
        self.score = score;
        self
    }

    pub fn force(mut self, force: bool) -> Self {
        self.force = Some(force);
        self
    }

    pub fn disable_edit_message(mut self, disable_edit_message: bool) -> Self {
        self.disable_edit_message = Some(disable_edit_message);
        self
    }

    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = Some(chat_id);
        self
    }

    pub fn message_id(mut self, message_id: i64) -> Self {
        self.message_id = Some(message_id);
        self
    }

    pub fn inline_message_id(mut self, inline_message_id: String) -> Self {
        self.inline_message_id = Some(inline_message_id);
        self
    }

    pub async fn send(self) -> Result<Message> {
        let form = serde_json::to_value(&self)?;
        self.bot.get("setGameScore", Some(&form)).await
    }
}
