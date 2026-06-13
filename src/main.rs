use actix_limitation::RateLimiter;
use actix_web::{App, HttpServer, middleware::Logger, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let pool = mrcntr::establish_connection();
    let shared_pool = web::Data::new(pool);

    let limiter = web::Data::new(mrcntr::build_limiter());

    let app = move || {
        App::new()
            .wrap(RateLimiter::default())
            .app_data(limiter.clone())
            .wrap(Logger::default())
            .app_data(shared_pool.clone())
            .configure(mrcntr::router::routes)
    };

    let ip = "127.0.0.1";
    let port = 8080;

    println!("App listening on http://{}:{}", ip, port);

    HttpServer::new(app).bind((ip, port))?.run().await
}
