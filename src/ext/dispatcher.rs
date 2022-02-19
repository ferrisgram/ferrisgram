use std::collections::HashMap;

use crate::Bot;
use crate::ext::Handler;

pub struct Dispatcher<'a> {
    pub bot: &'a Bot,
    handler_groups: Vec<i32>,
    handlers: HashMap<i32, Vec<Box<dyn Handler>>>
}

impl <'a> Dispatcher<'a> {
    pub fn new() {

    }
}