extern crate actix_web;
extern crate futures;
extern crate reqwest;
extern crate url;

pub mod r#async;
mod client;
mod error;
mod parameter;
mod response;

pub use self::client::Client;
pub use self::error::{Error, Result};
pub use self::parameter::Parameter;
pub use self::response::Response;
