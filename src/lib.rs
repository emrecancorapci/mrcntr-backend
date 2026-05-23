pub mod config {
    pub mod db;
    pub mod schema;
}

pub use config::db::DbPool;
pub use config::db::PooledConn;
pub use config::schema;
