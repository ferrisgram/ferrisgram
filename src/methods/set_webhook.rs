// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]
use serde::Serialize;

use crate::error::Result;
use crate::input_file::InputFile;
use crate::Bot;
use std::collections::HashMap;

impl Bot {
    /// Use this method to specify a URL and receive incoming updates via an outgoing webhook. Whenever there is an update for the bot, we will send an HTTPS POST request to the specified URL, containing a JSON-serialized Update. In case of an unsuccessful request, we will give up after a reasonable amount of attempts. Returns True on success.
    /// If you'd like to make sure that the webhook was set by you, you can specify secret data in the parameter secret_token. If specified, the request will contain a header "X-Telegram-Bot-Api-Secret-Token" with the secret token as content.
    /// <https://core.telegram.org/bots/api#setwebhook>
    pub fn set_webhook<F: InputFile>(&self, url: String) -> SetWebhookBuilder<F> {
        SetWebhookBuilder::new(self, url)
    }
}

#[derive(Serialize)]
pub struct SetWebhookBuilder<'a, F: InputFile> {
    #[serde(skip)]
    bot: &'a Bot,
    #[serde(skip)]
    data: HashMap<&'a str, F>,
    /// HTTPS URL to send updates to. Use an empty string to remove webhook integration
    pub url: String,
    /// The fixed IP address which will be used to send webhook requests instead of the IP address resolved through DNS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// The maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery, 1-100. Defaults to 40. Use lower values to limit the load on your bot's server, and higher values to increase your bot's throughput.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i64>,
    /// A JSON-serialized list of the update types you want your bot to receive. For example, specify ["message", "edited_channel_post", "callback_query"] to only receive updates of these types. See Update for a complete list of available update types. Specify an empty list to receive all update types except chat_member, message_reaction, and message_reaction_count (default). If not specified, the previous setting will be used. Please note that this parameter doesn't affect updates created before the call to the setWebhook, so unwanted updates may be received for a short period of time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
    /// Pass True to drop all pending updates
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_pending_updates: Option<bool>,
    /// A secret token to be sent in a header "X-Telegram-Bot-Api-Secret-Token" in every webhook request, 1-256 characters. Only characters A-Z, a-z, 0-9, _ and - are allowed. The header is useful to ensure that the request comes from a webhook set by you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_token: Option<String>,
}

impl<'a, F: InputFile> SetWebhookBuilder<'a, F> {
    pub fn new(bot: &'a Bot, url: String) -> Self {
        let data = HashMap::new();
        Self {
            bot,
            data,
            url,
            ip_address: None,
            max_connections: None,
            allowed_updates: None,
            drop_pending_updates: None,
            secret_token: None,
        }
    }

    pub fn url(mut self, url: String) -> Self {
        self.url = url;
        self
    }

    pub fn certificate(mut self, certificate: F) -> Self {
        self.data.insert("certificate", certificate);
        self
    }

    pub fn ip_address(mut self, ip_address: String) -> Self {
        self.ip_address = Some(ip_address);
        self
    }

    pub fn max_connections(mut self, max_connections: i64) -> Self {
        self.max_connections = Some(max_connections);
        self
    }

    pub fn allowed_updates(mut self, allowed_updates: Vec<String>) -> Self {
        self.allowed_updates = Some(allowed_updates);
        self
    }

    pub fn drop_pending_updates(mut self, drop_pending_updates: bool) -> Self {
        self.drop_pending_updates = Some(drop_pending_updates);
        self
    }

    pub fn secret_token(mut self, secret_token: String) -> Self {
        self.secret_token = Some(secret_token);
        self
    }

    pub async fn send(self) -> Result<bool> {
        let form = serde_json::to_value(&self)?;
        self.bot
            .post("setWebhook", Some(&form), Some(self.data))
            .await
    }
}
