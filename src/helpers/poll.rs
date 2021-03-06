// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::Poll;

impl Poll {
    /// This function creates an empty struct for the object Poll.
    pub fn new() -> Self {
        Self {
            id: "".to_string(),
            question: "".to_string(),
            options: Vec::new(),
            total_voter_count: 0,
            is_closed: false,
            is_anonymous: false,
            r#type: "".to_string(),
            allows_multiple_answers: false,
            correct_option_id: None,
            explanation: None,
            explanation_entities: None,
            open_period: None,
            close_date: None,
        }
    }
}
impl Default for Poll {
    fn default() -> Self {
        Self::new()
    }
}
