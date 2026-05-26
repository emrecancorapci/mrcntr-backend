use actix_web::{App, HttpServer, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = mrcntr::config::db::establish_connection();
    let shared_pool = web::Data::new(pool);

    let app = move || {
        App::new()
            .app_data(shared_pool.clone())
            .configure(mrcntr::router::routes)
    };

    let ip = "127.0.0.1";
    let port = 8080;

    println!("App listening on http://{}:{}", ip, port);

    HttpServer::new(app).bind((ip, port))?.run().await
}
