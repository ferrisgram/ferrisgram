use tokio::task;
use dyn_clone::{clone_trait_object, DynClone};

use crate::ext::Context;
use crate::types::Update;
use crate::{error::GroupIteration, error::Error, Bot};

pub type HandlerResult<T> = std::result::Result<T, Error>;

pub trait Handler: DynClone {
    fn check_update(&self, bot: &Bot, update: &Update) -> bool;
    fn handle_update(&self, bot: &Bot, context: &Context) -> task::JoinHandle<HandlerResult<GroupIteration>>;
}

clone_trait_object!(Handler);
