use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Mutex;

use crate::ext::Dispatcher;
use crate::types::Update;
use crate::{error, Bot};
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use lazy_static::lazy_static;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

lazy_static! {
    pub static ref CHANNEL: Mutex<(Sender<Update>, Receiver<Update>)> = Mutex::new(channel());
}

pub fn send(update: Update) {
    let x = CHANNEL.lock().unwrap();
    x.0.send(update).unwrap();
}

pub struct Updater<'a> {
    pub bot: &'a Bot,
    pub dispatcher: &'a mut Dispatcher<'a>,
    pub allowed_updates: Option<Vec<&'a str>>,
    polling: bool,
    webhook: bool,
    webhook_server: Option<tokio::task::JoinHandle<Result<(), std::io::Error>>>,
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
            webhook_server: None,
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
    pub async fn start_webhook(
        &mut self,
        addr: String,
        path: String,
        certificate: Option<String>,
        private_key: Option<String>,
        secret_token: Option<String>,
    ) -> error::Result<()> {
        self.webhook = true;
        if self.checker() {
            self.webhook_server = Some(tokio::task::spawn(async move {
                let http_server = HttpServer::new(move || {
                    App::new()
                        .route(&path, web::post().to(webhook_processor))
                        .app_data(secret_token.clone())
                });
                if private_key.is_some() && certificate.is_some() {
                    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
                    if let Some(p_key) = private_key {
                        builder
                            .set_private_key_file(p_key, SslFiletype::PEM)
                            .unwrap();
                    }
                    if let Some(cert) = certificate {
                        builder.set_certificate_chain_file(cert).unwrap();
                    }
                    http_server
                        .bind_openssl(&addr, builder)
                        .unwrap()
                        .run()
                        .await
                } else {
                    http_server.bind(&addr).unwrap().run().await
                }
            }));
            let x = CHANNEL.lock().unwrap();
            while self.webhook {
                let u_chan = x.1.recv();
                self.dispatcher.process_update(&u_chan.unwrap()).await;
            }
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
        if self.webhook_server.is_some() {
            self.webhook_server.as_ref().unwrap().abort();
        }
        self.webhook_server = None;
    }
}

async fn webhook_processor(
    request: HttpRequest,
    update: web::Json<Update>,
    tg_secret: Option<String>,
) -> impl Responder {
    if tg_secret.is_some() {
        let local_secret = tg_secret.unwrap();
        let recieved_secret = request
            .headers()
            .get("“X-Telegram-Bot-Api-Secret-Token")
            .unwrap()
            .to_str()
            .unwrap();
        if local_secret != recieved_secret {
            return "";
        }
    }
    send(update.0);
    ""
}
