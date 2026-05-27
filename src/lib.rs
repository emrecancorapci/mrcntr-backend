mod config {
    pub mod db;
    pub mod schema;
}

mod modules {
    pub mod experiences;
    pub mod tags;
}

pub mod router;

pub use config::db::*;
pub use config::schema;
