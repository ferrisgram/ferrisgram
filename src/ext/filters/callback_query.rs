use dyn_clone::{DynClone, clone_trait_object};

use crate::{types::CallbackQuery, filter_extension};

pub trait CallbackQueryFilter: Sync + Send + DynClone {
    fn check_filter(&self, m: &CallbackQuery) -> bool;
}
clone_trait_object!(CallbackQueryFilter);

#[derive(Clone)]
pub struct All {
    and_filter: Option<Box<dyn CallbackQueryFilter>>,
    or_filter: Option<Box<dyn CallbackQueryFilter>>,
    inverted: bool
}
impl CallbackQueryFilter for All {
    fn check_filter(&self, m: &CallbackQuery) -> bool {
        self.check_integral_filter(m, true)
    }
}
impl All {
    pub fn filter() -> Box<Self> {
        Box::new(
            Self{ 
                and_filter: None,
                or_filter: None,
                inverted: false,
            }
        )
    }
}
filter_extension!(All, CallbackQuery, dyn CallbackQueryFilter);


#[derive(Clone)]
pub struct ChatInstance {
    and_filter: Option<Box<dyn CallbackQueryFilter>>,
    or_filter: Option<Box<dyn CallbackQueryFilter>>,
    inverted: bool,
    instance: String
}
impl CallbackQueryFilter for ChatInstance {
    fn check_filter(&self, m: &CallbackQuery) -> bool {
        self.check_integral_filter(m, m.chat_instance == self.instance)
    }
}
impl ChatInstance {
    pub fn filter(instance: String) -> Box<Self> {
        Box::new(
            Self{ 
                and_filter: None,
                or_filter: None,
                inverted: false,
                instance
            }
        )
    }
}
filter_extension!(ChatInstance, CallbackQuery, dyn CallbackQueryFilter);


#[derive(Clone)]
pub struct Equal {
    and_filter: Option<Box<dyn CallbackQueryFilter>>,
    or_filter: Option<Box<dyn CallbackQueryFilter>>,
    inverted: bool,
    pub data: String
}
impl CallbackQueryFilter for Equal {
    fn check_filter(&self, m: &CallbackQuery) -> bool {
        self.check_integral_filter(m, 
            m.data.is_some() && m.data.as_ref().unwrap() == &self.data
        )
    }
}
impl Equal {
    pub fn filter(data: String) -> Box<Self> {
        Box::new(
            Self{
                and_filter: None,
                or_filter: None,
                inverted: false,
                data
            }
        )
    }
}
filter_extension!(Equal, CallbackQuery, dyn CallbackQueryFilter);


#[derive(Clone)]
pub struct FromUser {
    and_filter: Option<Box<dyn CallbackQueryFilter>>,
    or_filter: Option<Box<dyn CallbackQueryFilter>>,
    inverted: bool,
    pub user_id: i64
}
impl CallbackQueryFilter for FromUser {
    fn check_filter(&self, m: &CallbackQuery) -> bool {
        self.check_integral_filter(m, 
            m.from.id == self.user_id
        )
    }
}
impl FromUser {
    pub fn filter(user_id: i64) -> Box<Self> {
        Box::new(
            Self{
                and_filter: None,
                or_filter: None,
                inverted: false,
                user_id
            }
        )
    }
}
filter_extension!(FromUser, CallbackQuery, dyn CallbackQueryFilter);


#[derive(Clone)]
pub struct GameName {
    and_filter: Option<Box<dyn CallbackQueryFilter>>,
    or_filter: Option<Box<dyn CallbackQueryFilter>>,
    inverted: bool,
    pub name: String
}
impl CallbackQueryFilter for GameName {
    fn check_filter(&self, m: &CallbackQuery) -> bool {
        self.check_integral_filter(m, 
            m.game_short_name.is_some() && m.game_short_name.as_ref().unwrap() == &self.name
        )
    }
}
impl GameName {
    pub fn filter(name: String) -> Box<Self> {
        Box::new(
            Self{
                and_filter: None,
                or_filter: None,
                inverted: false,
                name
            }
        )
    }
}
filter_extension!(GameName, CallbackQuery, dyn CallbackQueryFilter);


#[derive(Clone)]
pub struct Inline {
    and_filter: Option<Box<dyn CallbackQueryFilter>>,
    or_filter: Option<Box<dyn CallbackQueryFilter>>,
    inverted: bool
}
impl CallbackQueryFilter for Inline {
    fn check_filter(&self, m: &CallbackQuery) -> bool {
        self.check_integral_filter(m, m.inline_message_id.is_some())
    }
}
impl Inline {
    pub fn filter() -> Box<Self> {
        Box::new(
            Self{ 
                and_filter: None,
                or_filter: None,
                inverted: false,
            }
        )
    }
}
filter_extension!(Inline, CallbackQuery, dyn CallbackQueryFilter);


#[derive(Clone)]
pub struct Prefix {
    and_filter: Option<Box<dyn CallbackQueryFilter>>,
    or_filter: Option<Box<dyn CallbackQueryFilter>>,
    inverted: bool,
    pub prefix: String
}
impl CallbackQueryFilter for Prefix {
    fn check_filter(&self, m: &CallbackQuery) -> bool {
        self.check_integral_filter(m, 
            m.data.is_some() && m.data.as_ref().unwrap().starts_with(&self.prefix)
        )
    }
}
impl Prefix {
    pub fn filter(prefix: String) -> Box<Self> {
        Box::new(
            Self{
                and_filter: None,
                or_filter: None,
                inverted: false,
                prefix
            }
        )
    }
}
filter_extension!(Prefix, CallbackQuery, dyn CallbackQueryFilter);


#[derive(Clone)]
pub struct Suffix {
    and_filter: Option<Box<dyn CallbackQueryFilter>>,
    or_filter: Option<Box<dyn CallbackQueryFilter>>,
    inverted: bool,
    pub suffix: String
}
impl CallbackQueryFilter for Suffix {
    fn check_filter(&self, m: &CallbackQuery) -> bool {
        self.check_integral_filter(m, 
            m.data.is_some() && m.data.as_ref().unwrap().ends_with(&self.suffix)
        )
    }
}
impl Suffix {
    pub fn filter(suffix: String) -> Box<Self> {
        Box::new(
            Self{
                and_filter: None,
                or_filter: None,
                inverted: false,
                suffix
            }
        )
    }
}
filter_extension!(Suffix, CallbackQuery, dyn CallbackQueryFilter);