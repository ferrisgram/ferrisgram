use async_trait::async_trait;
use dyn_clone::{clone_trait_object, DynClone};

use crate::{Bot, error};
use crate::types::Update;

#[async_trait]
pub trait Handler: Send + Sync + DynClone {
    async fn check_update(&self, bot: &Bot, update: &Update) -> bool;
    async fn handle_update(&self, bot: &Bot, update: &Update) -> error::Error;
}

clone_trait_object!(Handler);