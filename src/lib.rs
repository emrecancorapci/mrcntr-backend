mod config {
    pub mod db;
    pub mod schema;
}

pub mod middlewares {
    pub mod auth;
}

mod modules;

pub mod router;

pub use config::db::*;
pub use config::schema;
