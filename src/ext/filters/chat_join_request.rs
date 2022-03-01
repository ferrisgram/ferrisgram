use dyn_clone::{clone_trait_object, DynClone};

use crate::{filter_extension, types::ChatJoinRequest};

pub trait ChatJoinRequestFilter: Sync + Send + DynClone {
    fn check_filter(&self, m: &ChatJoinRequest) -> bool;
}
clone_trait_object!(ChatJoinRequestFilter);

#[derive(Clone)]
pub struct All {
    and_filter: Option<Box<dyn ChatJoinRequestFilter>>,
    or_filter: Option<Box<dyn ChatJoinRequestFilter>>,
    inverted: bool,
}
impl ChatJoinRequestFilter for All {
    fn check_filter(&self, m: &ChatJoinRequest) -> bool {
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
filter_extension!(All, ChatJoinRequest, dyn ChatJoinRequestFilter);

#[derive(Clone)]
pub struct FromUser {
    and_filter: Option<Box<dyn ChatJoinRequestFilter>>,
    or_filter: Option<Box<dyn ChatJoinRequestFilter>>,
    inverted: bool,
    pub user_id: i64,
}
impl ChatJoinRequestFilter for FromUser {
    fn check_filter(&self, m: &ChatJoinRequest) -> bool {
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
filter_extension!(FromUser, ChatJoinRequest, dyn ChatJoinRequestFilter);

#[derive(Clone)]
pub struct FromChat {
    and_filter: Option<Box<dyn ChatJoinRequestFilter>>,
    or_filter: Option<Box<dyn ChatJoinRequestFilter>>,
    inverted: bool,
    pub chat_id: i64,
}
impl ChatJoinRequestFilter for FromChat {
    fn check_filter(&self, m: &ChatJoinRequest) -> bool {
        self.check_integral_filter(m, m.chat.id == self.chat_id)
    }
}
impl FromChat {
    pub fn filter(chat_id: i64) -> Box<Self> {
        Box::new(Self {
            and_filter: None,
            or_filter: None,
            inverted: false,
            chat_id,
        })
    }
}
filter_extension!(FromChat, ChatJoinRequest, dyn ChatJoinRequestFilter);

#[derive(Clone)]
pub struct Group {
    and_filter: Option<Box<dyn ChatJoinRequestFilter>>,
    or_filter: Option<Box<dyn ChatJoinRequestFilter>>,
    inverted: bool,
}
impl ChatJoinRequestFilter for Group {
    fn check_filter(&self, m: &ChatJoinRequest) -> bool {
        self.check_integral_filter(m, m.chat.r#type == "group")
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
filter_extension!(Group, ChatJoinRequest, dyn ChatJoinRequestFilter);

#[derive(Clone)]
pub struct Private {
    and_filter: Option<Box<dyn ChatJoinRequestFilter>>,
    or_filter: Option<Box<dyn ChatJoinRequestFilter>>,
    inverted: bool,
}
impl ChatJoinRequestFilter for Private {
    fn check_filter(&self, m: &ChatJoinRequest) -> bool {
        self.check_integral_filter(m, m.chat.r#type == "private")
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
filter_extension!(Private, ChatJoinRequest, dyn ChatJoinRequestFilter);

#[derive(Clone)]
pub struct SuperGroup {
    and_filter: Option<Box<dyn ChatJoinRequestFilter>>,
    or_filter: Option<Box<dyn ChatJoinRequestFilter>>,
    inverted: bool,
}
impl ChatJoinRequestFilter for SuperGroup {
    fn check_filter(&self, m: &ChatJoinRequest) -> bool {
        self.check_integral_filter(m, m.chat.r#type == "supergroup")
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
filter_extension!(SuperGroup, ChatJoinRequest, dyn ChatJoinRequestFilter);
