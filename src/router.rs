use actix_web::{guard, web};

use crate::modules::experiences;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api").service(
            web::scope("/v1").service(
                web::scope("/experiences")
                    .service(experiences::many)
                    .service(experiences::one)
                    .service(
                        web::scope("")
                            .guard(guard::Header("content-type", "json/application"))
                            .service(experiences::insert)
                            .service(experiences::update)
                            .service(experiences::delete),
                    ),
            ),
        ),
    );
}
