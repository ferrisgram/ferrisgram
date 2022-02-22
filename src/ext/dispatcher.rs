use std::future::Future;

use crate::Bot;
use crate::ext::{Handler, Context};
use crate::error::{GroupIteration, Result, Error};
use GroupIteration::{ContinueGroups, EndGroups, ResumeGroups};
use crate::types::Update;

pub struct Dispatcher<'a, F: Future<Output = Result<GroupIteration>> + Send + 'static> {
    pub bot: &'a Bot,
    handler_groups: Vec<i32>,
    handlers: Vec<HandlersGroup>,
    error_handler: fn(Bot, Context, Error) -> F
}

#[derive(Clone)]
struct HandlersGroup {
    pub handler_group: i32,
    pub handlers: Vec<Box<dyn Handler>>
}

impl HandlersGroup {
    fn new(group: i32) -> Self {
        Self {
            handler_group: group,
            handlers: Vec::new(),
        }
    }
}

#[allow(unused_must_use)]
impl <'a, F: Future<Output = Result<GroupIteration>> + Send + 'static> Dispatcher<'a, F> {
    pub fn new(bot: &'a Bot, error_handler: fn(Bot, Context, Error) -> F) -> Self {
        let s = Self {
            bot,
            handler_groups: Vec::new(),
            handlers: Vec::new(),
            error_handler
        };
        s
    }
    
    pub fn add_handler_to_group(&mut self, handler_group: i32, handler_trait: Box<dyn Handler>) {
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
        self.handler_groups.sort()
    }
    pub fn add_handler(&mut self, handler: Box<dyn Handler>) {
        self.add_handler_to_group(0, handler)
    }
    pub async fn process_update(&mut self, update: &Update) {
        let ctx = Context::new(update);
        for group in self.handler_groups.iter() {
            for handler in self.handlers.iter() {
                if &handler.handler_group == group {
                    for handler in handler.handlers.iter() {
                        if !handler.check_update(self.bot, update).await {
                            continue;
                        }
                        let res = handler.handle_update(self.bot, &ctx).await;
                        match res {
                            Ok(mode) => {
                                match mode {
                                    EndGroups => return,
                                    ContinueGroups => break,
                                    ResumeGroups => continue,
                                }
                            },
                            Err(error) => {
                                (self.error_handler)(self.bot.clone(), ctx.clone(), error).await;
                            },
                        }
                    }
                }
            }
        }
    }
}
