use dyn_clone::{clone_trait_object, DynClone};

use crate::{filter_extension, types::Message};
pub trait MessageFilter: Sync + Send + DynClone {
    fn check_filter(&self, m: &Message) -> bool;
}
clone_trait_object!(MessageFilter);

macro_rules! simple_filter_maker {
    ($($t:ty)*) => {
        $(
            impl $t {
                #[inline]
                /// This method creates a filter from the object.
                pub fn filter() -> Box<Self> {
                    Box::new(
                        Self{
                            and_filter: None,
                            or_filter: None,
                            inverted: false
                        }
                    )
                }
            }
        )*
    }
}

simple_filter_maker! {
    All Animation SuperGroup Private
    Group Forwarded Caption VideoNote
    Text Video Sticker Command Audio
    Document Dice Contact Voice Photo
    CaptionEntities Entities ViaBot
    PinnedMessage LeftChatMember Game
    NewChatMembers Location Venue Poll
    IsAutomaticForward MediaGroup Migrate
    ReplyMarkup MigrateTo MigrateFrom
}

#[derive(Clone)]
pub struct All {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>,
    inverted: bool,
}
impl MessageFilter for All {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, true)
    }
}
filter_extension!(All, Message, dyn MessageFilter);

#[derive(Clone)]
pub struct SuperGroup {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>,
    inverted: bool,
}
impl MessageFilter for SuperGroup {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.chat.r#type == "supergroup")
    }
}
filter_extension!(SuperGroup, Message, dyn MessageFilter);

#[derive(Clone)]
pub struct Private {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>,
    inverted: bool,
}
impl MessageFilter for Private {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.chat.r#type == "private")
    }
}
filter_extension!(Private, Message, dyn MessageFilter);

#[derive(Clone)]
pub struct Group {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>,
    inverted: bool,
}
impl MessageFilter for Group {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.chat.r#type == "group")
    }
}
filter_extension!(Group, Message, dyn MessageFilter);

#[derive(Clone)]
pub struct Forwarded {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>,
    inverted: bool,
}
impl MessageFilter for Forwarded {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.forward_origin.is_some())
    }
}
filter_extension!(Forwarded, Message, dyn MessageFilter);

#[derive(Clone)]
pub struct Caption {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>,
    inverted: bool,
}
impl MessageFilter for Caption {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.caption.is_some())
    }
}
filter_extension!(Caption, Message, dyn MessageFilter);

#[derive(Clone)]
pub struct Reply {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>,
    inverted: bool,
}
impl MessageFilter for Reply {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.reply_to_message.is_some())
    }
}
filter_extension!(Reply, Message, dyn MessageFilter);

#[derive(Clone)]
pub struct Command {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>,
    inverted: bool,
}
impl MessageFilter for Command {
    fn check_filter(&self, m: &Message) -> bool {
        let mut checked_result = false;
        if m.entities.is_some() {
            let entities = m.entities.as_ref().unwrap();
            checked_result = entities[0].r#type == "bot_command" && entities[0].offset == 0
        }
        self.check_integral_filter(m, checked_result)
    }
}
filter_extension!(Command, Message, dyn MessageFilter);

#[derive(Clone)]
pub struct Text {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>,
    inverted: bool,
}
impl MessageFilter for Text {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.text.is_some())
    }
}
filter_extension!(Text, Message, dyn MessageFilter);

#[derive(Clone)]
pub struct Animation {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>,
    inverted: bool,
}
impl MessageFilter for Animation {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.animation.is_some())
    }
}
filter_extension!(Animation, Message, dyn MessageFilter);

#[derive(Clone)]
pub struct Video {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>,
    inverted: bool,
}
impl MessageFilter for Video {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.video.is_some())
    }
}
filter_extension!(Video, Message, dyn MessageFilter);

#[derive(Clone)]
pub struct Sticker {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>,
    inverted: bool,
}
impl MessageFilter for Sticker {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.sticker.is_some())
    }
}
filter_extension!(Sticker, Message, dyn MessageFilter);

#[derive(Clone)]
pub struct Photo {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>,
    inverted: bool,
}
impl MessageFilter for Photo {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.photo.is_some())
    }
}
filter_extension!(Photo, Message, dyn MessageFilter);

#[derive(Clone)]
pub struct Audio {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>,
    inverted: bool,
}
impl MessageFilter for Audio {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.audio.is_some())
    }
}
filter_extension!(Audio, Message, dyn MessageFilter);

#[derive(Clone)]
pub struct Document {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>,
    inverted: bool,
}
impl MessageFilter for Document {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.document.is_some())
    }
}
filter_extension!(Document, Message, dyn MessageFilter);

#[derive(Clone)]
pub struct Dice {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>,
    inverted: bool,
}
impl MessageFilter for Dice {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.dice.is_some())
    }
}
filter_extension!(Dice, Message, dyn MessageFilter);

#[derive(Clone)]
pub struct Contact {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>,
    inverted: bool,
}
impl MessageFilter for Contact {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.contact.is_some())
    }
}
filter_extension!(Contact, Message, dyn MessageFilter);

#[derive(Clone)]
pub struct Voice {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>,
    inverted: bool,
}
impl MessageFilter for Voice {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.voice.is_some())
    }
}
filter_extension!(Voice, Message, dyn MessageFilter);

#[derive(Clone)]
pub struct VideoNote {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>,
    inverted: bool,
}
impl MessageFilter for VideoNote {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.video_note.is_some())
    }
}
filter_extension!(VideoNote, Message, dyn MessageFilter);

#[derive(Clone)]
pub struct FromUser {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>,
    inverted: bool,
    pub user_id: i64,
}
impl MessageFilter for FromUser {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(
            m,
            m.from.is_some() && m.from.as_ref().unwrap().id == self.user_id,
        )
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
filter_extension!(FromUser, Message, dyn MessageFilter);

#[derive(Clone)]
pub struct FromChat {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>,
    inverted: bool,
    pub chat_id: i64,
}
impl MessageFilter for FromChat {
    fn check_filter(&self, m: &Message) -> bool {
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
filter_extension!(FromChat, Message, dyn MessageFilter);

#[derive(Clone)]
pub struct DiceValue {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>,
    inverted: bool,
    pub value: i64,
}
impl MessageFilter for DiceValue {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(
            m,
            m.dice.is_some() && m.dice.as_ref().unwrap().value == self.value,
        )
    }
}
impl DiceValue {
    pub fn filter(value: i64) -> Box<Self> {
        Box::new(Self {
            and_filter: None,
            or_filter: None,
            inverted: false,
            value,
        })
    }
}
filter_extension!(DiceValue, Message, dyn MessageFilter);

#[derive(Clone)]
pub struct Game {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>,
    inverted: bool,
}
impl MessageFilter for Game {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.game.is_some())
    }
}
filter_extension!(Game, Message, dyn MessageFilter);

#[derive(Clone)]
pub struct Poll {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>,
    inverted: bool,
}
impl MessageFilter for Poll {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.poll.is_some())
    }
}
filter_extension!(Poll, Message, dyn MessageFilter);

#[derive(Clone)]
pub struct Venue {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>,
    inverted: bool,
}
impl MessageFilter for Venue {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.venue.is_some())
    }
}
filter_extension!(Venue, Message, dyn MessageFilter);

#[derive(Clone)]
pub struct Location {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>,
    inverted: bool,
}
impl MessageFilter for Location {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.location.is_some())
    }
}
filter_extension!(Location, Message, dyn MessageFilter);

#[derive(Clone)]
pub struct NewChatMembers {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>,
    inverted: bool,
}
impl MessageFilter for NewChatMembers {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.new_chat_members.is_some())
    }
}
filter_extension!(NewChatMembers, Message, dyn MessageFilter);

#[derive(Clone)]
pub struct LeftChatMember {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>,
    inverted: bool,
}
impl MessageFilter for LeftChatMember {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.left_chat_member.is_some())
    }
}
filter_extension!(LeftChatMember, Message, dyn MessageFilter);

#[derive(Clone)]
pub struct PinnedMessage {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>,
    inverted: bool,
}
impl MessageFilter for PinnedMessage {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.pinned_message.is_some())
    }
}
filter_extension!(PinnedMessage, Message, dyn MessageFilter);

#[derive(Clone)]
pub struct ViaBot {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>,
    inverted: bool,
}
impl MessageFilter for ViaBot {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.via_bot.is_some())
    }
}
filter_extension!(ViaBot, Message, dyn MessageFilter);

#[derive(Clone)]
pub struct Entities {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>,
    inverted: bool,
}
impl MessageFilter for Entities {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.entities.is_some())
    }
}
filter_extension!(Entities, Message, dyn MessageFilter);

#[derive(Clone)]
pub struct CaptionEntities {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>,
    inverted: bool,
}
impl MessageFilter for CaptionEntities {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.caption_entities.is_some())
    }
}
filter_extension!(CaptionEntities, Message, dyn MessageFilter);

#[derive(Clone)]
pub struct Migrate {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>,
    inverted: bool,
}
impl MessageFilter for Migrate {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(
            m,
            m.migrate_from_chat_id.is_some() || m.migrate_to_chat_id.is_some(),
        )
    }
}
filter_extension!(Migrate, Message, dyn MessageFilter);

#[derive(Clone)]
pub struct MigrateFrom {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>,
    inverted: bool,
}
impl MessageFilter for MigrateFrom {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.migrate_from_chat_id.is_some())
    }
}
filter_extension!(MigrateFrom, Message, dyn MessageFilter);

#[derive(Clone)]
pub struct MigrateTo {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>,
    inverted: bool,
}
impl MessageFilter for MigrateTo {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.migrate_to_chat_id.is_some())
    }
}
filter_extension!(MigrateTo, Message, dyn MessageFilter);

#[derive(Clone)]
pub struct ReplyMarkup {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>,
    inverted: bool,
}
impl MessageFilter for ReplyMarkup {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.reply_markup.is_some())
    }
}
filter_extension!(ReplyMarkup, Message, dyn MessageFilter);

#[derive(Clone)]
pub struct MediaGroup {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>,
    inverted: bool,
}
impl MessageFilter for MediaGroup {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.media_group_id.is_some())
    }
}
filter_extension!(MediaGroup, Message, dyn MessageFilter);

#[derive(Clone)]
pub struct IsAutomaticForward {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>,
    inverted: bool,
}
impl MessageFilter for IsAutomaticForward {
    fn check_filter(&self, m: &Message) -> bool {
        self.check_integral_filter(m, m.is_automatic_forward.is_some())
    }
}
filter_extension!(IsAutomaticForward, Message, dyn MessageFilter);

#[derive(Clone)]
pub struct Entity {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>,
    inverted: bool,
    pub en_type: String,
}
impl MessageFilter for Entity {
    fn check_filter(&self, m: &Message) -> bool {
        let mut checked_result = false;
        if m.entities.is_some() {
            for en in m.entities.as_ref().unwrap() {
                if en.r#type == self.en_type {
                    checked_result = true;
                }
            }
        }
        self.check_integral_filter(m, checked_result)
    }
}
impl Entity {
    pub fn filter(en_type: String) -> Box<Self> {
        Box::new(Self {
            and_filter: None,
            or_filter: None,
            inverted: false,
            en_type,
        })
    }
}
filter_extension!(Entity, Message, dyn MessageFilter);

#[derive(Clone)]
pub struct CaptionEntity {
    and_filter: Option<Box<dyn MessageFilter>>,
    or_filter: Option<Box<dyn MessageFilter>>,
    inverted: bool,
    pub en_type: String,
}
impl MessageFilter for CaptionEntity {
    fn check_filter(&self, m: &Message) -> bool {
        let mut checked_result = false;
        if m.caption_entities.is_some() {
            for en in m.caption_entities.as_ref().unwrap() {
                if en.r#type == self.en_type {
                    checked_result = true;
                }
            }
        }
        self.check_integral_filter(m, checked_result)
    }
}
impl CaptionEntity {
    pub fn filter(en_type: String) -> Box<Self> {
        Box::new(Self {
            and_filter: None,
            or_filter: None,
            inverted: false,
            en_type,
        })
    }
}
filter_extension!(CaptionEntity, Message, dyn MessageFilter);
