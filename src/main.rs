use actix_limitation::RateLimiter;
use actix_web::{App, HttpServer, middleware::Logger, web};
use mrcntr::{AppDatabase, AppLimiter};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = diesel_async::pooled_connection::AsyncDieselConnectionManager::<
        diesel_async::AsyncPgConnection,
    >::new(database_url);

    let db_pool = diesel_async::pooled_connection::deadpool::Pool::builder(config)
        .build()
        .expect("Failed to create Database connection pool");

    let shared_db_pool = web::Data::new(db_pool);

    let redis_pool = AppDatabase::redis_conn();
    let shared_redis_pool = web::Data::new(redis_pool);

    let limiter = AppLimiter::build();
    let shared_limiter = web::Data::new(limiter);

    let app = move || {
        App::new()
            .wrap(RateLimiter::default())
            .app_data(shared_limiter.clone())
            .wrap(Logger::default())
            .app_data(shared_db_pool.clone())
            .app_data(shared_redis_pool.clone())
            .configure(mrcntr::router::routes)
    };

    let ip = "127.0.0.1";
    let port = 8080;

    println!("App listening on http://{}:{}", ip, port);

    HttpServer::new(app).bind((ip, port))?.run().await
}
