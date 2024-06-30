use crate::error::{Error, GroupIteration};
use crate::ext::{Context, Handler};
use crate::types::Update;
use crate::Bot;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use GroupIteration::{ContinueGroups, EndGroups, ResumeGroups};

type ErrorHandlerFunc = fn(&Bot, &Context, Error) -> GroupIteration;

#[derive(Clone)]
pub struct Dispatcher<'a> {
    pub bot: &'a Bot,
    handler_groups: Vec<i32>,
    handlers: Vec<HandlersGroup>,
    error_handler: ErrorHandlerFunc,
}

#[derive(Clone)]
struct HandlersGroup {
    pub handler_group: i32,
    pub handlers: Vec<Box<dyn Handler>>,
}

impl HandlersGroup {
    fn new(group: i32) -> Self {
        Self {
            handler_group: group,
            handlers: Vec::new(),
        }
    }
}
use std::sync::Arc;

async fn webhook_callback(
    // request: HttpRequest,
    update: web::Json<Update>,
    bd: web::Data<Bot>,
    hgd: web::Data<Vec<i32>>,
    hd: web::Data<Vec<HandlersGroup>>,
    ehd: web::Data<ErrorHandlerFunc>,
) -> impl Responder {
    let bot = bd.into_inner();
    let groups = hgd.into_inner();
    let handlers = hd.into_inner();
    let error_handler = ehd.into_inner();
    let update = Arc::new(update.into_inner());
    process_update(groups, handlers, error_handler, bot, update);
    HttpResponse::Ok()
}

fn process_update(
    handler_groups: Arc<Vec<i32>>,
    handlers: Arc<Vec<HandlersGroup>>,
    error_handler: Arc<ErrorHandlerFunc>,
    bot: Arc<Bot>,
    update: Arc<Update>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let ctx = Context::new(&update);
        for group in handler_groups.iter() {
            for handler in handlers.iter() {
                if &handler.handler_group == group {
                    for handler in handler.handlers.iter() {
                        if !handler.check_update(&bot, &update).await {
                            continue;
                        }
                        let res = handler.handle_update(&bot, &ctx).await;
                        match res {
                            Ok(mode) => match mode {
                                EndGroups => return,
                                ContinueGroups => break,
                                ResumeGroups => continue,
                            },
                            Err(error) => {
                                (error_handler)(&bot, &ctx, error);
                            }
                        }
                    }
                }
            }
        }
    })
}

#[allow(unused_must_use)]
impl<'a> Dispatcher<'a> {
    pub fn new(bot: &'a Bot) -> Self {
        Self {
            bot,
            handler_groups: Vec::new(),
            handlers: Vec::new(),
            error_handler: Self::default_error_handler,
        }
    }

    pub fn add_handler_to_group(&mut self, handler_trait: Box<dyn Handler>, handler_group: i32) {
        if !self.handler_groups.contains(&handler_group) {
            self.handler_groups.push(handler_group);
            self.handlers.push(HandlersGroup::new(handler_group))
        }
        let mut handlers = Vec::new();
        for mut hg in self.handlers.clone() {
            if hg.handler_group == handler_group {
                hg.handlers.push(handler_trait.clone())
            }
            handlers.push(hg)
        }
        self.handlers = handlers;
        self.handler_groups.sort_unstable()
    }
    pub fn add_handler(&mut self, handler: Box<dyn Handler>) {
        self.add_handler_to_group(handler, 0)
    }
    pub fn add_error_handler(&mut self, error_hander: ErrorHandlerFunc) {
        self.error_handler = error_hander
    }
    pub(crate) async fn start_webhook(
        &mut self,
        port: u16,
    ) -> std::result::Result<(), std::io::Error> {
        let handler_groups = self.handler_groups.clone();
        let handlers = self.handlers.clone();
        let error_handler = self.error_handler;
        let bot = self.bot.clone();
        HttpServer::new(move || {
            App::new()
                .route("/", web::get().to(webhook_callback))
                .app_data(web::Data::new(handler_groups.clone()))
                .app_data(web::Data::new(handlers.clone()))
                .app_data(web::Data::new(error_handler.clone()))
                .app_data(web::Data::new(bot.clone()))
        })
        .bind(("127.0.0.1", port))?
        .run()
        .await
    }
    pub async fn process_update(&mut self, update: &Update) -> tokio::task::JoinHandle<()> {
        let handler_groups = Arc::new(self.handler_groups.clone());
        let handlers = Arc::new(self.handlers.clone());
        let error_handler = Arc::new(self.error_handler);
        let bot = Arc::new(self.bot.clone());
        let update = Arc::new(update.clone());
        process_update(handler_groups, handlers, error_handler, bot, update)
    }
    fn default_error_handler(_: &Bot, _: &Context, error: Error) -> GroupIteration {
        println!("{}", error);
        ResumeGroups
    }
}
