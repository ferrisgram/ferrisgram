use async_trait::async_trait;
use dyn_clone::{clone_trait_object, DynClone};

use crate::{Bot, error::GroupIteration, error::Result};
use crate::types::Update;
use crate::ext::Context;

#[async_trait]
pub trait Handler: Send + Sync + DynClone {
    async fn check_update(&self, bot: &Bot, context: &Update) -> bool;
    async fn handle_update(&self, bot: &Bot, context: &Context) -> Result<GroupIteration>;
}

clone_trait_object!(Handler);