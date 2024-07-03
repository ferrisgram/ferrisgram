use std::sync::Arc;

use crate::types::Update;
use crate::{error, Bot};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

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
}

impl<'a> Updater<'a> {
    pub fn new(bot: Arc<Bot>, dispatcher: &'a mut dispatcher::Dispatcher) -> Self {
        Self {
            running: false,
            allowed_updates: None,
            dispatcher,
            bot,
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
    pub async fn start_webhook(&mut self, port: u16) -> std::result::Result<(), std::io::Error> {
        let dp = self.dispatcher.clone();
        HttpServer::new(move || {
            App::new()
                .route("/", web::get().to(webhook_callback))
                // .app_data(web::Data::new(handler_groups.clone()))
                // .app_data(web::Data::new(handlers.clone()))
                // .app_data(web::Data::new(error_handler.clone()))
                // .app_data(web::Data::new(bot.clone()))
                .app_data(web::Data::new(dp.clone()))
        })
        .bind(("127.0.0.1", port))?
        .run()
        .await
    }
    pub async fn stop(&mut self) {
        self.running = false;
    }
}
