use actix_web::{guard, web};

use crate::modules::experiences;

pub fn routes(cfg: &mut web::ServiceConfig) {
    let v1 = web::scope("/v1");

    // Experiences
    let v1 = v1
        .service(
                web::scope("/experiences")
                    .service(experiences::many)
                .service(experiences::one),
        )
                    .service(
            web::scope("/experiences")
                            .guard(guard::Header("content-type", "json/application"))
                            .service(experiences::insert)
                            .service(experiences::update)
                            .service(experiences::delete),
    );

    cfg.service(web::scope("/api").service(v1));
}
