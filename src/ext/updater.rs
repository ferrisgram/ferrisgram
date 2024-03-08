use crate::ext::Dispatcher;
use crate::{error, Bot};
use crate::types::Update;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::sync::Arc;

pub struct Updater<'a> {
    pub bot: &'a Bot,
    pub dispatcher: &'a mut Dispatcher<'a>,
    pub allowed_updates: Option<Vec<&'a str>>,
    polling: bool,
    webhook: bool,
    webhook_handler: Option<tokio::task::JoinHandle<Result<(), std::io::Error>>>,
}

impl<'a> Updater<'a> {
    fn checker(&mut self) -> bool {
        if self.polling || self.webhook {
            self.polling ^ self.webhook
        } else {
            true
        }
    }
    pub fn new(bot: &'a Bot, dispatcher: &'a mut Dispatcher<'a>) -> Self {
        Self {
            polling: false,
            webhook: false,
            allowed_updates: None,
            dispatcher,
            bot,
            webhook_handler: None,
        }
    }
    pub async fn start_polling(&mut self, drop_pending_updates: bool) -> error::Result<()> {
        let mut offset = if drop_pending_updates { -2 } else { 0 };
        self.polling = true;
        let allowed_updates: Option<Vec<String>> = if self.allowed_updates.is_some() {
            Some(
                self.allowed_updates
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            )
        } else {
            None
        };
        if self.checker() {
            while self.polling {
                let mut updates = self.bot.get_updates().offset(offset + 1).timeout(10);
                if allowed_updates.is_some() {
                    updates = updates.allowed_updates(allowed_updates.clone().unwrap());
                }
                for update in updates.send().await?.iter() {
                    offset = update.update_id;
                    self.dispatcher.process_update(update).await;
                }
            }
            Ok(())
        } else {
            Err(error::Error::Conflict(
                "start_polling and start_webhook cannot be called at the same time".to_string(),
            ))
        }
    }
    // THis is where i work
    /* A breifing ...
     *
     * A server which will wait for telegram to send a request. (Will try actix .. if not working will switch to hyper)
     * Both polling and webhook function cannot be running together DONE
     * The recieved request will be verified with a telegram secret token
     * The recieved data will be phrased to Update data type ...
     * Gets the cerificate from the user and uses it for starting the server DONE
     * (I guess this much for now)
     *
     * Settings
     * A url .. where to start the server
     * A certificate .. start the server with certs
     * secret_token to compare with incoming telegram requests
     */
    pub async fn start_webhook(
        &mut self,
        addr: String,
        path: String,
        certificate: &str,
        private_key: &str,
        secret_token: Option<String>,
    ) -> error::Result<()> {
        self.webhook = true;
        if self.checker() {
            let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
            builder
                .set_private_key_file(private_key, SslFiletype::PEM)
                .unwrap();
            builder.set_certificate_chain_file(&certificate).unwrap();
            //let actix_webhook=Arc::<Dispatcher>::new(*self.dispatcher);
            self.webhook_handler = Some(tokio::task::spawn(async move {
                HttpServer::new(
                    move || {
                        App::new()
                            .route(&path, web::post().to(webhook_processor))
                            .app_data(secret_token.clone())
                    }, //.app_data(&dispa)
                )
                .bind_openssl(&addr, builder)
                .unwrap()
                .run()
                .await
            }));
            Ok(())
        } else {
            Err(error::Error::Conflict(
                "start_polling and start_webhook cannot be used at the same time".to_string(),
            ))
        }
    }
    pub async fn stop(&mut self) {
        self.polling = false;
        self.webhook = false;
        if self.webhook_handler.is_some() {
            self.webhook_handler.as_ref().unwrap().abort();
        }
        self.webhook_handler = None;
    }
}
async fn webhook_processor(
    request:HttpRequest,
    updater:web::Json<Update>,
    tg_secret:Option<String>) -> impl Responder {

    if tg_secret.is_some() {
        let secret=tg_secret.unwrap();
        let recieved=request.headers().get("“X-Telegram-Bot-Api-Secret-Token").unwrap().to_str().unwrap();
        if secret==recieved {
            //if you get dispacter here just put the variable inside the process_update and done
            "auth"
        } else {
            "wrong"
        }
    } else {
        "no auth"
    }
}
