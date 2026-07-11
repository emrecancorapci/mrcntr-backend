use actix_limitation::RateLimiter;
use actix_web::{App, HttpServer, middleware::Logger, web};
use mrcntr::{ApiDoc, AppDatabase, AppLimiter};
use utoipa::OpenApi;
use utoipa_actix_web::AppExt;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let db_pool = AppDatabase::db_conn();
    let shared_db_pool = web::Data::new(db_pool);

    let redis_pool = AppDatabase::redis_conn();
    let shared_redis_pool = web::Data::new(redis_pool);

    let limiter = AppLimiter::build();
    let shared_limiter = web::Data::new(limiter);

    let app = move || {
        App::new()
            .into_utoipa_app()
            .openapi(ApiDoc::openapi())
            .map(|app| {
                app.wrap(RateLimiter::default())
                    .app_data(shared_limiter.clone())
                    .wrap(Logger::default())
                    .app_data(shared_db_pool.clone())
                    .app_data(shared_redis_pool.clone())
            })
            .service(
                utoipa_actix_web::scope("/api")
                    .service(utoipa_actix_web::scope("/v1").configure(mrcntr::router::routes)),
            )
            .openapi_service(|api| {
                SwaggerUi::new("/docs/{_:.*}").url("/docs/openapi.json", api)
            })
            .into_app()
    };

    let ip = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    println!("App listening on http://{}:{}", ip, port);

    HttpServer::new(app).bind((ip, port))?.run().await
}
