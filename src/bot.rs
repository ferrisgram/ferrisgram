use std::time::Duration;

use crate::error::{Error, Result};
use crate::types::User;
use crate::DEFAULT_API_URL;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Bot {
    pub token: String,
    pub client: Client,
    pub api_url: String,
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse<T> {
    pub ok: bool,
    pub error_code: Option<i32>,
    pub description: Option<String>,
    pub result: Option<T>,
}

impl Bot {
    pub async fn new(token: &str) -> Result<Bot> {
        let client = Client::builder()
            .timeout(Duration::from_secs(5 * 60 + 30))
            .connect_timeout(Duration::from_secs(60))
            .build()
            .unwrap();
        let mut bot = Bot {
            token: String::from(token),
            client,
            api_url: DEFAULT_API_URL.to_string(),
            user: User::new(),
        };
        match bot.get_me().send().await {
            Ok(bot_user) => {
                bot.user = bot_user;
                Ok(bot)
            }
            Err(_) => Err(Error::InvalidToken),
        }
    }
    pub async fn get<T: DeserializeOwned>(
        &self,
        method: &str,
        params: Option<&serde_json::Value>,
    ) -> Result<T> {
        let mut resp = self.client.get(format!(
            "{url}/bot{token}/{method}",
            url = self.api_url,
            token = self.token,
            method = method
        ));
        if params.is_some() {
            resp = resp.json(&params);
        }
        let resp = resp.send().await?.json::<ApiResponse<T>>().await?;
        if resp.ok {
            Ok(resp.result.unwrap())
        } else {
            Err(Error::TelegramError(resp.description.unwrap()))
        }
    }
    pub async fn post<T: DeserializeOwned>(
        &self,
        method: &str,
        params: Option<&serde_json::Value>,
    ) -> Result<T> {
        let mut resp = self.client.post(format!(
            "{url}/bot{token}/{method}",
            url = self.api_url,
            token = self.token,
            method = method
        ));
        if params.is_some() {
            resp = resp.json(&params);
        }
        let resp = resp.send().await?.json::<ApiResponse<T>>().await?;
        if resp.ok {
            Ok(resp.result.unwrap())
        } else {
            Err(Error::TelegramError(resp.description.unwrap()))
        }
    }
}
