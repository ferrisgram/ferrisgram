use std::sync::Arc;
use std::time::Duration;

use crate::types::Update;
use crate::{error, Bot};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use tokio::task::AbortHandle;
use tokio::time::sleep;

use super::dispatcher;

async fn webhook_callback(
    // request: HttpRequest,
    update: web::Json<Box<Update>>,
    dp: web::Data<dispatcher::Dispatcher>,
) -> impl Responder {
    dp.process_update(&update).await;
    HttpResponse::Ok()
}

pub struct Updater<'a> {
    pub bot: Arc<Bot>,
    pub dispatcher: &'a mut dispatcher::Dispatcher,
    pub allowed_updates: Option<Vec<&'a str>>,
    running: bool,
    abort_handle: Option<AbortHandle>
}

impl<'a> Updater<'a> {
    pub fn new(bot: Arc<Bot>, dispatcher: &'a mut dispatcher::Dispatcher) -> Self {
        Self {
            running: false,
            allowed_updates: None,
            dispatcher,
            bot,
            abort_handle: None,
        }
    }
    pub async fn start_polling(&mut self, drop_pending_updates: bool) -> error::Result<()> {
        let mut offset = 0;
        if drop_pending_updates {
            offset = -2;
        }
        self.running = true;
        let mut allowed_updates: Option<Vec<String>> = None;
        if self.allowed_updates.is_some() {
            allowed_updates = Some(
                self.allowed_updates
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|s| s.to_string())
                    .collect(),
            );
        }
        let mut pending_tasks = Vec::new();
        while self.running {
            let mut updates = self.bot.get_updates().offset(offset + 1).timeout(10);
            if allowed_updates.is_some() {
                updates = updates.allowed_updates(allowed_updates.clone().unwrap());
            }
            for update in updates.send().await?.iter() {
                offset = update.update_id;
                let task = self.dispatcher.process_update(update).await;
                pending_tasks.push(task);
            }
        }
        for task in pending_tasks {
            let _ = task.await;
        }
        Ok(())
    }
    pub async fn start_webhook(
        &mut self,
        port: u16,
        opts: StartWebhookOpts,
    ) {
        let dp = self.dispatcher.clone();
        let path: String;
        if opts.path.is_some() {
            path = opts.path.unwrap();
        } else {
            path = String::from("/");
        }
        let addrs = ("127.0.0.1", port);
        let task = tokio::spawn(async move {
            let http_server = HttpServer::new(move || {
                App::new()
                    .route(&path, web::get().to(webhook_callback))
                    .app_data(web::Data::new(dp.clone()))
            });
            if opts.certificate.is_some() && opts.private_key.is_some() {
                let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
                if let Some(p_key) = opts.private_key {
                    builder
                        .set_private_key_file(p_key, SslFiletype::PEM)
                        .unwrap();
                }
                if let Some(cert) = opts.certificate {
                    builder.set_certificate_chain_file(cert).unwrap();
                }
                http_server.bind_openssl(addrs, builder).unwrap().run().await
            } else {
                http_server.bind(addrs).unwrap().run().await
            }
        });
        self.abort_handle = Some(task.abort_handle());
        while self.running {
            sleep(Duration::from_secs(1)).await;
        }
    }
    pub async fn stop(mut self) {
        self.running = false;
        if let Some(task) = self.abort_handle {
            task.abort();
        }
    }
}

pub struct StartWebhookOpts {
    path: Option<String>,
    private_key: Option<String>,
    certificate: Option<String>,
}

impl StartWebhookOpts {
    pub fn new() -> Self {
        Self{
            path: None,
            private_key: None,
            certificate: None,
        }
    }
    pub fn path(mut self, path: &str) -> Self {
        self.path = Some(path.to_string());
        self
    }
    pub fn private_key(mut self, private_key: String) -> Self {
        self.private_key = Some(private_key);
        self
    }
    pub fn certificate(mut self, certificate: String) -> Self {
        self.certificate = Some(certificate);
        self
    }
}
