// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::error::Result;
use crate::Bot;

impl Bot {
    /// Use this method to remove webhook integration if you decide to switch back to getUpdates. Returns True on success.
    /// <https://core.telegram.org/bots/api#deletewebhook>
    pub fn delete_webhook(&self) -> DeleteWebhookBuilder {
        DeleteWebhookBuilder::new(self)
    }
}

#[derive(Serialize)]
pub struct DeleteWebhookBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Pass True to drop all pending updates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_pending_updates: Option<bool>,
}

impl<'a> DeleteWebhookBuilder<'a> {
    pub fn new(bot: &'a Bot) -> Self {
        Self {
            bot,
            drop_pending_updates: None,
        }
    }

    pub fn drop_pending_updates(mut self, drop_pending_updates: bool) -> Self {
        self.drop_pending_updates = Some(drop_pending_updates);
        self
    }

    pub async fn send(self) -> Result<bool> {
        let form = serde_json::to_value(&self)?;
        self.bot.get("deleteWebhook", Some(&form)).await
    }
}
