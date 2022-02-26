#![allow(non_upper_case_globals)]

use dyn_clone::{DynClone, clone_trait_object};

use crate::types::Message;

macro_rules! integral_filter {
    ($($t:ty)*) => {
        $(
            impl $t {
                #[inline]
                pub fn and(mut self, filter: Box<dyn MessageFilter>) -> Box<Self> {
                    self.and_filter = Some(filter);
                    Box::from(self)
                }
                pub fn or(mut self, filter: Box<dyn MessageFilter>) -> Box<Self> {
                    self.or_filter = Some(filter);
                    Box::from(self)
                }
                pub fn check_integral_filter(&self, m: &Message, parent_result: bool) -> bool {
                    if self.and_filter.is_none() && self.or_filter.is_none() {
                        return parent_result
                    }
                    (
                        parent_result 
                        && 
                        self.and_filter.is_some() 
                        && 
                        self.and_filter.as_ref().unwrap().check_filter(m)
                    ) || (
                        self.or_filter.is_some() 
                        && 
                        self.or_filter.as_ref().unwrap().check_filter(m)
                    )
            
                }
                pub fn filter() -> Box<Self> {
                    Box::new(
                        Self{ 
                            and_filter: None,
                            or_filter: None,
                        }
                    )
                }
            }
        )*
    }
}

integral_filter!{
    All 
    Animation
    SuperGroup 
    Private 
    Group 
    Forwarded
    Caption
    Command
    Text
    Video
    Sticker
    Photo
    Audio
    Document
    Dice
    Contact
    Voice
    VideoNote
}

pub trait MessageFilter: Sync + Send + DynClone {
    fn check_filter(&self, m: &Message) -> bool;
}
clone_trait_object!(MessageFilter);


#[derive(Clone)]
pub struct All {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>
}
impl MessageFilter for All {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, true)
    }
}


#[derive(Clone)]
pub struct SuperGroup {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>
}
impl MessageFilter for SuperGroup {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.chat.r#type == "supergroup")
    }
}


#[derive(Clone)]
pub struct Private {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>
}
impl MessageFilter for Private {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.chat.r#type == "private")
    }
}


#[derive(Clone)]
pub struct Group {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>
}
impl MessageFilter for Group {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.chat.r#type == "group")
    }
}


#[derive(Clone)]
pub struct Forwarded {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>
}
impl MessageFilter for Forwarded {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.forward_date.is_some())
    }
}


#[derive(Clone)]
pub struct Caption {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>
}
impl MessageFilter for Caption {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.caption.is_some())
    }
}


// #TODO
#[derive(Clone)]
pub struct Command {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>
}
impl MessageFilter for Command {
    fn check_filter(&self, _: &Message) -> bool {
        true
    }
}

#[derive(Clone)]
pub struct Text {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>
}
impl MessageFilter for Text {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.text.is_some())
    }
}


#[derive(Clone)]
pub struct Animation {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>
}
impl MessageFilter for Animation {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.animation.is_some())
    }
}


#[derive(Clone)]
pub struct Video {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>
}
impl MessageFilter for Video {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.video.is_some())
    }
}


#[derive(Clone)]
pub struct Sticker {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>
}
impl MessageFilter for Sticker {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.sticker.is_some())
    }
}


#[derive(Clone)]
pub struct Photo {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>
}
impl MessageFilter for Photo {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.photo.is_some())
    }
}


#[derive(Clone)]
pub struct Audio {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>
}
impl MessageFilter for Audio {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.audio.is_some())
    }
}


#[derive(Clone)]
pub struct Document {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>
}
impl MessageFilter for Document {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.document.is_some())
    }
}


#[derive(Clone)]
pub struct Dice {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>
}
impl MessageFilter for Dice {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.dice.is_some())
    }
}


#[derive(Clone)]
pub struct Contact {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>
}
impl MessageFilter for Contact {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.contact.is_some())
    }
}


#[derive(Clone)]
pub struct Voice {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>
}
impl MessageFilter for Voice {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.voice.is_some())
    }
}


#[derive(Clone)]
pub struct VideoNote {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>
}
impl MessageFilter for VideoNote {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.video_note.is_some())
    }
}