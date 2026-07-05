#![warn(clippy::all)]
#![allow(unused_parens)]

mod config {
    pub mod db;
    pub mod error_handler;
    pub mod limiter;
    pub mod schema;
}

pub mod middlewares {
    pub mod auth;
}

mod modules;

pub mod macros;
pub mod router;

pub use config::db::*;
pub use config::error_handler::AppError;
pub use config::limiter::*;
pub use config::schema;
