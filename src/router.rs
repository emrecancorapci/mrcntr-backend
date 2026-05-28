use actix_web::{guard, web};

use crate::modules::{experiences, tags};

pub fn routes(cfg: &mut web::ServiceConfig) {
    let v1 = web::scope("/v1");

    // Experiences
    let v1 = v1.service(
        web::scope("/experiences")
            .service(experiences::many)
            .service(experiences::one)
            .service(experiences::insert)
            .service(experiences::update)
            .service(experiences::delete),
    );

    // Tags
    let v1 = v1.service(
        web::scope("/tags")
            .service(tags::many)
            .service(tags::one)
            .service(tags::insert)
            .service(tags::update)
            .service(tags::delete),
    );

    cfg.service(web::scope("/api").service(v1));
}
