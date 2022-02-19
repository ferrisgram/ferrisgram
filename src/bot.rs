use std::time::Duration;

use reqwest::Client;
use crate::DEFAULT_API_URL;
use crate::error::{Result, Error};
use crate::types::{User};
use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;

#[derive(Debug, Clone)]
pub struct Bot {
    pub token: String,
    pub client: Client,
    pub api_url: String,
    pub user: User
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse<T> {
    pub ok: bool,
    pub error_code: Option<i32>,
    pub description: Option<String>,
    pub result: Option<T>
}

impl Bot {
    pub async fn get<T: DeserializeOwned>(&self, method: &str, params: Option<&serde_json::Value>) -> Result<T> {
        let mut resp = self.client.get(
            format!("{url}/bot{token}/{method}", url=self.api_url,  token=self.token, method=method)
        );
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
    pub async fn post<T: DeserializeOwned>(&self, method: &str, params: Option<&serde_json::Value>) -> Result<T> {
        let mut resp = self.client.post(
            format!("{url}/bot{token}/{method}", url=self.api_url,  token=self.token, method=method)
        );
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

pub async fn new_bot(token: &str) -> Result<Bot> {
    let client = Client::builder()
            .timeout(Duration::from_secs(5 * 60 + 30))
            .connect_timeout(Duration::from_secs(60))
            .build()
            .unwrap();
    let mut bot = Bot {
        token:String::from(token), 
        client,
        api_url: DEFAULT_API_URL.to_string(),
        user: User { 
            id: 0, 
            is_bot: false, 
            first_name: "".to_string(), 
            last_name: None, 
            username: None, 
            language_code: None, 
            can_join_groups: None, 
            can_read_all_group_messages: None, 
            supports_inline_queries: None 
        },
    };
    match bot.get_me().send().await {
        Ok(bot_user) => {
            bot.user = bot_user;
            Ok(bot)
        }
        Err(_) => Err(Error::InvalidToken)
    }
}