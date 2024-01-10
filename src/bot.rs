use crate::error::{Error, Result};
use crate::input_file::InputFile;
use crate::types::User;
use crate::DEFAULT_API_URL;
use reqwest::multipart::Form;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;

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

// pub struct NamedFile {
//     pub file_name: String,
//     pub file_data: Vec<u8>,
// }

// impl InputFile for NamedFile {
//     fn get_part(&self) -> Part {
//         // let ext = Path::new(&self.file_name).extension().and_then(|ext| ext.to_str()).unwrap_or("");
//         // let mime = mime_guess::from_ext(ext).first_or_octet_stream();
//         Part::bytes(self.file_data.clone()).file_name(self.file_name.clone())
//     }
// }

// pub trait InputFile {
//     fn get_part(&self) -> Part;
// }

impl Bot {
    pub async fn new(token: &str, api_url: Option<&str>) -> Result<Bot> {
        let mut api = DEFAULT_API_URL;
        if api_url.is_some() {
            api = api_url.unwrap();
        }
        let mut bot = Bot {
            token: String::from(token),
            client: Client::builder()
                .timeout(Duration::from_secs(5 * 60 + 30))
                .connect_timeout(Duration::from_secs(60))
                .build()
                .unwrap(),
            api_url: api.to_string(),
            user: User::new(0, true, String::new()),
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
    pub async fn post<T: DeserializeOwned, F: InputFile>(
        &self,
        method: &str,
        params: Option<&serde_json::Value>,
        data: Option<HashMap<&str, F>>,
    ) -> Result<T> {
        let mut resp = self.client.post(format!(
            "{url}/bot{token}/{method}",
            url = self.api_url,
            token = self.token,
            method = method
        ));
        // Use multipart if data is not none else use JSON
        println!("Adding multipart form");
        if data.is_some() {
            let mut form = Form::new();
            for (name, field) in data.unwrap() {
                form = form.part(name.to_string(), field.get_part())
            }
            println!("Adding params");
            if let Value::Object(obj) = params.unwrap().clone() {
                for (key, val) in obj {
                    if let Value::String(s) = val {
                        form = form.text(key, s);
                    } else {
                        form = form.text(key, val.to_string())
                    }
                }
            }
            resp = resp.multipart(form);
            // form.part("", part);
        } else if params.is_some() {
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
