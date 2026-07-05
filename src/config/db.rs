use deadpool_redis::{Config, Runtime};
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::Pool;

pub type DbPool = Pool<AsyncPgConnection>;
pub type PooledConn = AsyncPgConnection;
pub type RedisPool = deadpool_redis::Pool;

pub const REDIS_USER_TOKEN: &str = "users:tokens:";
pub const REDIS_USER_DATA: &str = "users:data:";

pub struct AppDatabase;

impl AppDatabase {
    pub fn db_conn() -> DbPool {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let config = AsyncDieselConnectionManager::new(database_url);

        Pool::builder(config)
            .build()
            .expect("Failed to create Database connection pool")
    }

    pub fn redis_conn() -> RedisPool {
        let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
        let config = Config::from_url(redis_url);

        config
            .create_pool(Some(Runtime::Tokio1))
            .expect("Failed to create Redis connection pool")
    }
}
