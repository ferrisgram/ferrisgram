// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::error::Result;
use crate::types::WebhookInfo;
use crate::Bot;

impl Bot {
    /// Use this method to get current webhook status. Requires no parameters. On success, returns a WebhookInfo object. If the bot is using getUpdates, will return an object with the url field empty.
    /// <https://core.telegram.org/bots/api#getwebhookinfo>
    pub fn get_webhook_info(&self) -> GetWebhookInfoBuilder {
        GetWebhookInfoBuilder::new(self)
    }
}

#[derive(Serialize)]
pub struct GetWebhookInfoBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
}

impl<'a> GetWebhookInfoBuilder<'a> {
    pub fn new(bot: &'a Bot) -> Self {
        Self { bot }
    }

    pub async fn send(self) -> Result<WebhookInfo> {
        let form = serde_json::to_value(&self)?;
        self.bot.get("getWebhookInfo", Some(&form)).await
    }
}
