use deadpool_redis::{Config, Runtime};
use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, PooledConnection},
};

pub type DbPool = diesel::r2d2::Pool<ConnectionManager<PgConnection>>;
pub type RedisPool = deadpool_redis::Pool;
pub type PooledConn = PooledConnection<ConnectionManager<PgConnection>>;

pub struct AppDatabase;

impl AppDatabase {
    pub fn redis_conn() -> RedisPool {
        let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");
        let config = Config::from_url(redis_url);

        config
            .create_pool(Some(Runtime::Tokio1))
            .expect("Failed to create Redis connection pool")
    }
}
