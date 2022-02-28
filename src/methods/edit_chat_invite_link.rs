// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::error::Result;
use crate::types::ChatInviteLink;
use crate::Bot;

impl Bot {
    /// Use this method to edit a non-primary invite link created by the bot. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns the edited invite link as a ChatInviteLink object.
    /// <https://core.telegram.org/bots/api#editchatinvitelink>
    pub fn edit_chat_invite_link(
        &self,
        chat_id: i64,
        invite_link: String,
    ) -> EditChatInviteLinkBuilder {
        EditChatInviteLinkBuilder::new(self, chat_id, invite_link)
    }
}

#[derive(Serialize)]
pub struct EditChatInviteLinkBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: i64,
    /// The invite link to edit
    pub invite_link: String,
    /// Invite link name; 0-32 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Point in time (Unix timestamp) when the link will expire
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<i64>,
    /// Maximum number of users that can be members of the chat simultaneously after joining the chat via this invite link; 1-99999
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<i64>,
    /// True, if users joining the chat via the link need to be approved by chat administrators. If True, member_limit can't be specified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creates_join_request: Option<bool>,
}

impl<'a> EditChatInviteLinkBuilder<'a> {
    pub fn new(bot: &'a Bot, chat_id: i64, invite_link: String) -> Self {
        Self {
            bot,
            chat_id,
            invite_link,
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

    pub fn invite_link(mut self, invite_link: String) -> Self {
        self.invite_link = invite_link;
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
        self.bot
            .get::<ChatInviteLink>("editChatInviteLink", Some(&form))
            .await
    }
}
