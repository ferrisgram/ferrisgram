use std::future::Future;
use async_trait::async_trait;

use crate::ext::filters::InlineQueryFilter;
use crate::ext::{Context, Handler};
use crate::types::Update;
use crate::{Bot, error::GroupIteration, error::Result};

pub struct InlineQueryHandler<F: Future<Output = Result<GroupIteration>> + Send + 'static> {
    pub callback: fn(Bot, Context) -> F,
    pub filter: Box<dyn InlineQueryFilter>
}

impl <F: Future<Output = Result<GroupIteration>> + Send + 'static> InlineQueryHandler<F> {
    pub fn new(callback: fn(Bot, Context) -> F, filter: Box<dyn InlineQueryFilter>) -> Box<Self> {
        Box::new(Self {
            callback,
            filter
        })
    }
}

impl<F: Future<Output = Result<GroupIteration>> + Send + 'static> Clone for InlineQueryHandler<F> {
    fn clone(&self) -> Self {
        Self {
            callback: self.callback,
            filter: self.filter.clone()
        }
    }
}

#[async_trait]
impl<F: Future<Output = Result<GroupIteration>> + Send + 'static> Handler for InlineQueryHandler<F> {
    async fn check_update(&self, _: &Bot, update: &Update) -> bool {
        if update.inline_query.is_none() {
            return false
        }
        self.filter.check_filter(update.inline_query.as_ref().unwrap())
    }
    async fn handle_update(&self, bot: &Bot, context: &Context) -> Result<GroupIteration> {
        (self.callback)(bot.clone(), context.clone()).await
    }
}
