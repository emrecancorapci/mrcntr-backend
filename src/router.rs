use actix_web::web;

use crate::modules::experiences::{delete, insert, many, one, update};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api").service(
            web::scope("/experiences").service(
                web::scope("/v1")
                    .service(many)
                    .service(one)
                    .service(insert)
                    .service(update)
                    .service(delete),
            ),
        ),
    );
}
