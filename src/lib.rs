#![warn(clippy::all)]
#![allow(unused_parens)]

mod config {
    pub mod error_handler;
    pub mod db;
    pub mod schema;
    pub mod limiter;
}

pub mod middlewares {
    pub mod auth;
}

mod modules;

pub mod macros;
pub mod router;

pub use config::db::*;
pub use config::schema;
pub use config::limiter::*;