// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use serde::{Deserialize, Serialize};


/// This object represents a service message about a video chat scheduled in the chat.
/// <https://core.telegram.org/bots/api#videochatscheduled>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoChatScheduled {
    /// Point in time (Unix timestamp) when the video chat is supposed to be started by a chat administrator
    pub start_date: i64,
}