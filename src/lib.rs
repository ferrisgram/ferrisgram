pub mod types;
pub mod methods;
pub mod ext;
pub mod error;
pub mod bot;
pub use bot::*;
mod helpers;

pub static DEFAULT_API_URL: &str = "https://api.telegram.org";