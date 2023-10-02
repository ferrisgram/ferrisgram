mod spec_types;
mod types;
mod methods;
mod helpers;
mod common;


const SCHEMA_URL: &str = "https://github.com/PaulSonOfLars/telegram-bot-api-spec/raw/main/api.json";

#[tokio::main]
pub async fn main() {
    match reqwest::get(SCHEMA_URL).await.unwrap().json::<spec_types::ApiDescription>().await {
        Ok(res) => {
            types::generate_types(&res).await;
            methods::generate_methods(&res).await;
            helpers::generate_helpers(&res).await;
        },
        Err(err) => println!("failed to parse json: {:?}", err),
    }   
}