use actix_web::web;

use crate::modules::{categories, experiences, experiences_tags, tags};

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

    let v1 = v1.service(
        web::scope("/experiences-tags")
            .service(experiences_tags::insert_one)
            .service(experiences_tags::insert_many)
            .service(experiences_tags::replace_many_by_experience_id)
            .service(experiences_tags::delete),
    );

    let v1 = v1.service(
        web::scope("/categories")
            .service(categories::many)
            .service(categories::one)
            .service(categories::insert)
            .service(categories::update)
            .service(categories::delete),
    );

    cfg.service(web::scope("/api").service(v1));
}
