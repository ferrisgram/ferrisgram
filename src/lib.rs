pub mod bot;
pub mod error;
pub mod ext;
pub mod methods;
pub mod types;
pub use bot::*;
mod helpers;

pub static DEFAULT_API_URL: &str = "https://api.telegram.org";
