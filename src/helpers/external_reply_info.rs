// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments, clippy::new_without_default)]
use crate::types::ExternalReplyInfo;
use crate::types::MessageOrigin;

impl ExternalReplyInfo {
    /// This function creates an empty struct for the object ExternalReplyInfo.
    pub fn new(origin: MessageOrigin) -> Self {
        Self {
            origin,
            chat: None,
            message_id: None,
            link_preview_options: None,
            animation: None,
            audio: None,
            document: None,
            photo: None,
            sticker: None,
            story: None,
            video: None,
            video_note: None,
            voice: None,
            has_media_spoiler: None,
            contact: None,
            dice: None,
            game: None,
            giveaway: None,
            giveaway_winners: None,
            invoice: None,
            location: None,
            poll: None,
            venue: None,
        }
    }
}
