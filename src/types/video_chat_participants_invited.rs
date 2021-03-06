// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::User;
use serde::{Deserialize, Serialize};

/// This object represents a service message about new members invited to a video chat.
/// <https://core.telegram.org/bots/api#videochatparticipantsinvited>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoChatParticipantsInvited {
    /// New members that were invited to the video chat
    pub users: Vec<User>,
}
