// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

#![allow(clippy::too_many_arguments)]use serde::Serialize;

use crate::Bot;
use crate::error::Result;
use crate::types::UserProfilePhotos;

impl Bot {
    /// Use this method to get a list of profile pictures for a user. Returns a UserProfilePhotos object.
    /// <https://core.telegram.org/bots/api#getuserprofilephotos>
    pub fn get_user_profile_photos(&self, user_id: i64) -> GetUserProfilePhotosBuilder {
        GetUserProfilePhotosBuilder::new(self, user_id)
    }
}

#[derive(Serialize)]
pub struct GetUserProfilePhotosBuilder<'a> {
    #[serde(skip)]
    bot: &'a Bot,
    /// Unique identifier of the target user
    pub user_id: i64,
    /// Sequential number of the first photo to be returned. By default, all photos are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Limits the number of photos to be retrieved. Values between 1-100 are accepted. Defaults to 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}


impl <'a> GetUserProfilePhotosBuilder<'a> {
    pub fn new(bot: &'a Bot, user_id: i64) -> Self {
        Self{
            bot,
            user_id,
            offset: None,
            limit: None,
        }
    }

    pub fn user_id(mut self, user_id: i64) -> Self {
        self.user_id = user_id;
        self
    }
                
    pub fn offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }
                
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
                
    pub async fn send(self) -> Result<UserProfilePhotos> {
        let form = serde_json::to_value(&self)?;
        self.bot.get::<UserProfilePhotos>("getUserProfilePhotos", Some(&form)).await
    }

}