use std::future::Future;
use async_trait::async_trait;

use crate::ext::{Context, Handler};
use crate::types::Update;
use crate::{Bot, error::GroupIteration, error::Result};

pub struct CallbackQueryHandler<F: Future<Output = Result<GroupIteration>> + Send + 'static> {
    pub callback: fn(Bot, Context) -> F
}

impl <F: Future<Output = Result<GroupIteration>> + Send + 'static> CallbackQueryHandler<F> {
    pub fn new(callback: fn(Bot, Context) -> F) -> Box<Self> {
        Box::new(Self {
            callback
        })
    }
}

impl<F: Future<Output = Result<GroupIteration>> + Send + 'static> Clone for CallbackQueryHandler<F> {
    fn clone(&self) -> Self {
        Self {
            callback: self.callback.clone(),
        }
    }
}

#[async_trait]
impl<F: Future<Output = Result<GroupIteration>> + Send + 'static> Handler for CallbackQueryHandler<F> {
    async fn check_update(&self, _: &Bot, update: &Update) -> bool {
        if update.callback_query.is_some() {
            return true
        }
        false
    }
    async fn handle_update(&self, bot: &Bot, context: &Context) -> Result<GroupIteration> {
        (self.callback)(bot.clone(), context.clone()).await
    }
}
