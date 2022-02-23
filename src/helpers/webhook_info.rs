// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::WebhookInfo;

impl WebhookInfo {
    /// This function creates an empty struct for the object WebhookInfo.
    pub fn new() -> Self {
        Self {
            url: "".to_string(),
            has_custom_certificate: false,
            pending_update_count: 0,
            ip_address: None,
            last_error_date: None,
            last_error_message: None,
            max_connections: None,
            allowed_updates: None,
        }
    }
}