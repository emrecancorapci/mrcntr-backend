pub mod config {
    pub mod db;
    pub mod schema;
}

pub mod modules {
    pub mod experiences;
    pub mod tags;
}

pub use config::db::DbPool;
pub use config::db::PooledConn;
pub use config::schema;
