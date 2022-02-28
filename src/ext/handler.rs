use async_trait::async_trait;
use dyn_clone::{clone_trait_object, DynClone};

use crate::ext::Context;
use crate::types::Update;
use crate::{error::GroupIteration, error::Result, Bot};

#[async_trait]
pub trait Handler: Send + Sync + DynClone {
    async fn check_update(&self, bot: &Bot, update: &Update) -> bool;
    async fn handle_update(&self, bot: &Bot, context: &Context) -> Result<GroupIteration>;
}

clone_trait_object!(Handler);
