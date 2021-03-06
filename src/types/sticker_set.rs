// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::{PhotoSize, Sticker};
use serde::{Deserialize, Serialize};

/// This object represents a sticker set.
/// <https://core.telegram.org/bots/api#stickerset>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StickerSet {
    /// Sticker set name
    pub name: String,
    /// Sticker set title
    pub title: String,
    /// True, if the sticker set contains animated stickers
    pub is_animated: bool,
    /// True, if the sticker set contains video stickers
    pub is_video: bool,
    /// True, if the sticker set contains masks
    pub contains_masks: bool,
    /// List of all set stickers
    pub stickers: Vec<Sticker>,
    /// Optional. Sticker set thumbnail in the .WEBP, .TGS, or .WEBM format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<PhotoSize>,
}
