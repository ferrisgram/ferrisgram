// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::error::Result;
use crate::Bot;

impl Bot {
    /// Use this method to clear the list of pinned messages in a forum topic. The bot must be an administrator in the chat for this to work and must have the can_pin_messages administrator right in the supergroup. Returns True on success.
    /// <https://core.telegram.org/bots/api#unpinallforumtopicmessages>
    pub fn unpin_all_forum_topic_messages(
        &self,
        chat_id: i64,
        message_thread_id: i64,
    ) -> UnpinAllForumTopicMessagesBuilder {
        UnpinAllForumTopicMessagesBuilder::new(self, chat_id, message_thread_id)
    }
}

#[derive(Serialize)]
pub struct UnpinAllForumTopicMessagesBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: i64,
    /// Unique identifier for the target message thread of the forum topic
    pub message_thread_id: i64,
}

impl<'a> UnpinAllForumTopicMessagesBuilder<'a> {
    pub fn new(bot: &'a Bot, chat_id: i64, message_thread_id: i64) -> Self {
        Self {
            bot,
            chat_id,
            message_thread_id,
        }
    }

    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }

    pub fn message_thread_id(mut self, message_thread_id: i64) -> Self {
        self.message_thread_id = message_thread_id;
        self
    }

    pub async fn send(self) -> Result<bool> {
        let form = serde_json::to_value(&self)?;
        self.bot
            .get("unpinAllForumTopicMessages", Some(&form))
            .await
    }
}
