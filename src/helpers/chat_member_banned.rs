// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::ChatMemberBanned;
use crate::types::User;

impl ChatMemberBanned {
    /// This function creates an empty struct for the object ChatMemberBanned.
    pub fn new() -> Self {
        Self {
            status: "".to_string(),
            user: User::new(),
            until_date: 0,
        }
    }
}