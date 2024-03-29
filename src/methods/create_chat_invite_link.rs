// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::error::Result;
use crate::types::ChatInviteLink;
use crate::Bot;

impl Bot {
    /// Use this method to create an additional invite link for a chat. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. The link can be revoked using the method revokeChatInviteLink. Returns the new invite link as ChatInviteLink object.
    /// <https://core.telegram.org/bots/api#createchatinvitelink>
    pub fn create_chat_invite_link(&self, chat_id: i64) -> CreateChatInviteLinkBuilder {
        CreateChatInviteLinkBuilder::new(self, chat_id)
    }
}

#[derive(Serialize)]
pub struct CreateChatInviteLinkBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
    /// Invite link name; 0-32 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Point in time (Unix timestamp) when the link will expire
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<i64>,
    /// The maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<i64>,
    /// True, if users joining the chat via the link need to be approved by chat administrators. If True, member_limit can't be specified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creates_join_request: Option<bool>,
}

impl<'a> CreateChatInviteLinkBuilder<'a> {
    pub fn new(bot: &'a Bot, chat_id: i64) -> Self {
        Self {
            bot,
            chat_id,
            name: None,
            expire_date: None,
            member_limit: None,
            creates_join_request: None,
        }
    }

    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn expire_date(mut self, expire_date: i64) -> Self {
        self.expire_date = Some(expire_date);
        self
    }

    pub fn member_limit(mut self, member_limit: i64) -> Self {
        self.member_limit = Some(member_limit);
        self
    }

    pub fn creates_join_request(mut self, creates_join_request: bool) -> Self {
        self.creates_join_request = Some(creates_join_request);
        self
    }

    pub async fn send(self) -> Result<ChatInviteLink> {
        let form = serde_json::to_value(&self)?;
        self.bot.get("createChatInviteLink", Some(&form)).await
    }
}
