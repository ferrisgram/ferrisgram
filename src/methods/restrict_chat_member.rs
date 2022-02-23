// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use serde::Serialize;

use crate::Bot;
use crate::error::Result;
use crate::types::ChatPermissions;

impl Bot {
    /// Use this method to restrict a user in a supergroup. The bot must be an administrator in the supergroup for this to work and must have the appropriate administrator rights. Pass True for all permissions to lift restrictions from a user. Returns True on success.
    /// <https://core.telegram.org/bots/api#restrictchatmember>
    pub fn restrict_chat_member(&self, chat_id: i64, user_id: i64, permissions: ChatPermissions) -> RestrictChatMemberBuilder {
        RestrictChatMemberBuilder::new(&self, chat_id, user_id, permissions)
    }
}

#[derive(Serialize)]
pub struct RestrictChatMemberBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    pub chat_id: i64,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// A JSON-serialized object for new user permissions
    pub permissions: ChatPermissions,
    /// Date when restrictions will be lifted for the user, unix time. If user is restricted for more than 366 days or less than 30 seconds from the current time, they are considered to be restricted forever
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,
}


impl <'a> RestrictChatMemberBuilder<'a> {
    pub fn new(bot: &'a Bot, chat_id: i64, user_id: i64, permissions: ChatPermissions) -> Self {
        Self{
            bot,
            chat_id,
            user_id,
            permissions,
            until_date: None,
        }
    }

    pub fn chat_id(mut self, chat_id: i64) -> Self {
        self.chat_id = chat_id;
        self
    }
                
    pub fn user_id(mut self, user_id: i64) -> Self {
        self.user_id = user_id;
        self
    }
                
    pub fn permissions(mut self, permissions: ChatPermissions) -> Self {
        self.permissions = permissions;
        self
    }
                
    pub fn until_date(mut self, until_date: i64) -> Self {
        self.until_date = Some(until_date);
        self
    }
                
    pub async fn send(self) -> Result<bool> {
        let form = serde_json::to_value(&self)?;
        self.bot.get::<bool>("restrictChatMember", Some(&form)).await
    }

}