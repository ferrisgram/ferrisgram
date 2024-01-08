// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::WebAppInfo;
use serde::{Deserialize, Serialize};

/// Represents a menu button, which launches a Web App.
/// <https://core.telegram.org/bots/api#menubuttonwebapp>
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MenuButtonWebApp {
    /// Text on the button
    pub text: String,
    /// Description of the Web App that will be launched when the user presses the button. The Web App will be able to send an arbitrary message on behalf of the user using the method answerWebAppQuery.
    pub web_app: WebAppInfo,
}
