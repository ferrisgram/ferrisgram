use std::time::Duration;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use tokio::time::sleep;

use crate::types::Update;

use super::{dispatcher, Updater};

async fn webhook_callback(
    // request: HttpRequest,
    update: web::Json<Box<Update>>,
    dp: web::Data<dispatcher::Dispatcher>,
) -> impl Responder {
    dp.process_update(&update).await;
    HttpResponse::Ok()
}

impl<'a> Updater<'a> {
    pub async fn start_webhook(&mut self, port: u16, opts: StartWebhookOpts) {
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
                http_server
                    .bind_openssl(addrs, builder)
                    .unwrap()
                    .run()
                    .await
            } else {
                http_server.bind(addrs).unwrap().run().await
            }
        });
        self.abort_handle = Some(task.abort_handle());
        loop {
            if !self.running {
                break;
            }
            sleep(Duration::from_secs(1)).await;
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
        Self {
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
