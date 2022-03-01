use dyn_clone::{clone_trait_object, DynClone};

use crate::{filter_extension, types::InlineQuery};

pub trait InlineQueryFilter: Sync + Send + DynClone {
    fn check_filter(&self, m: &InlineQuery) -> bool;
}
clone_trait_object!(InlineQueryFilter);

#[derive(Clone)]
pub struct All {
    and_filter: Option<Box<dyn InlineQueryFilter>>,
    or_filter: Option<Box<dyn InlineQueryFilter>>,
    inverted: bool,
}
impl InlineQueryFilter for All {
    fn check_filter(&self, m: &InlineQuery) -> bool {
        self.check_integral_filter(m, true)
    }
}
impl All {
    pub fn filter() -> Box<Self> {
        Box::new(Self {
            and_filter: None,
            or_filter: None,
            inverted: false,
        })
    }
}
filter_extension!(All, InlineQuery, dyn InlineQueryFilter);

#[derive(Clone)]
pub struct Equal {
    and_filter: Option<Box<dyn InlineQueryFilter>>,
    or_filter: Option<Box<dyn InlineQueryFilter>>,
    inverted: bool,
    pub data: String,
}
impl InlineQueryFilter for Equal {
    fn check_filter(&self, m: &InlineQuery) -> bool {
        self.check_integral_filter(m, m.query == self.data)
    }
}
impl Equal {
    pub fn filter(data: String) -> Box<Self> {
        Box::new(Self {
            and_filter: None,
            or_filter: None,
            inverted: false,
            data,
        })
    }
}
filter_extension!(Equal, InlineQuery, dyn InlineQueryFilter);

#[derive(Clone)]
pub struct FromUser {
    and_filter: Option<Box<dyn InlineQueryFilter>>,
    or_filter: Option<Box<dyn InlineQueryFilter>>,
    inverted: bool,
    pub user_id: i64,
}
impl InlineQueryFilter for FromUser {
    fn check_filter(&self, m: &InlineQuery) -> bool {
        self.check_integral_filter(m, m.from.id == self.user_id)
    }
}
impl FromUser {
    pub fn filter(user_id: i64) -> Box<Self> {
        Box::new(Self {
            and_filter: None,
            or_filter: None,
            inverted: false,
            user_id,
        })
    }
}
filter_extension!(FromUser, InlineQuery, dyn InlineQueryFilter);

#[derive(Clone)]
pub struct Group {
    and_filter: Option<Box<dyn InlineQueryFilter>>,
    or_filter: Option<Box<dyn InlineQueryFilter>>,
    inverted: bool,
}
impl InlineQueryFilter for Group {
    fn check_filter(&self, m: &InlineQuery) -> bool {
        self.check_integral_filter(
            m,
            m.chat_type.is_some() && m.chat_type.as_ref().unwrap() == "group",
        )
    }
}
impl Group {
    pub fn filter() -> Box<Self> {
        Box::new(Self {
            and_filter: None,
            or_filter: None,
            inverted: false,
        })
    }
}
filter_extension!(Group, InlineQuery, dyn InlineQueryFilter);

#[derive(Clone)]
pub struct Prefix {
    and_filter: Option<Box<dyn InlineQueryFilter>>,
    or_filter: Option<Box<dyn InlineQueryFilter>>,
    inverted: bool,
    pub prefix: String,
}
impl InlineQueryFilter for Prefix {
    fn check_filter(&self, m: &InlineQuery) -> bool {
        self.check_integral_filter(m, m.query.starts_with(&self.prefix))
    }
}
impl Prefix {
    pub fn filter(prefix: String) -> Box<Self> {
        Box::new(Self {
            and_filter: None,
            or_filter: None,
            inverted: false,
            prefix,
        })
    }
}
filter_extension!(Prefix, InlineQuery, dyn InlineQueryFilter);

#[derive(Clone)]
pub struct Private {
    and_filter: Option<Box<dyn InlineQueryFilter>>,
    or_filter: Option<Box<dyn InlineQueryFilter>>,
    inverted: bool,
}
impl InlineQueryFilter for Private {
    fn check_filter(&self, m: &InlineQuery) -> bool {
        self.check_integral_filter(
            m,
            m.chat_type.is_some() && m.chat_type.as_ref().unwrap() == "private",
        )
    }
}
impl Private {
    pub fn filter() -> Box<Self> {
        Box::new(Self {
            and_filter: None,
            or_filter: None,
            inverted: false,
        })
    }
}
filter_extension!(Private, InlineQuery, dyn InlineQueryFilter);

#[derive(Clone)]
pub struct Suffix {
    and_filter: Option<Box<dyn InlineQueryFilter>>,
    or_filter: Option<Box<dyn InlineQueryFilter>>,
    inverted: bool,
    pub suffix: String,
}
impl InlineQueryFilter for Suffix {
    fn check_filter(&self, m: &InlineQuery) -> bool {
        self.check_integral_filter(m, m.query.ends_with(&self.suffix))
    }
}
impl Suffix {
    pub fn filter(suffix: String) -> Box<Self> {
        Box::new(Self {
            and_filter: None,
            or_filter: None,
            inverted: false,
            suffix,
        })
    }
}
filter_extension!(Suffix, InlineQuery, dyn InlineQueryFilter);

#[derive(Clone)]
pub struct SuperGroup {
    and_filter: Option<Box<dyn InlineQueryFilter>>,
    or_filter: Option<Box<dyn InlineQueryFilter>>,
    inverted: bool,
}
impl InlineQueryFilter for SuperGroup {
    fn check_filter(&self, m: &InlineQuery) -> bool {
        self.check_integral_filter(
            m,
            m.chat_type.is_some() && m.chat_type.as_ref().unwrap() == "supergroup",
        )
    }
}
impl SuperGroup {
    pub fn filter() -> Box<Self> {
        Box::new(Self {
            and_filter: None,
            or_filter: None,
            inverted: false,
        })
    }
}
filter_extension!(SuperGroup, InlineQuery, dyn InlineQueryFilter);
