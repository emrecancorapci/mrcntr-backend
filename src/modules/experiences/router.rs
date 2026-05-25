use actix_web::web;

use super::handlers::{delete, insert, many, one, update};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/experiences")
            .service(many)
            .service(one)
            .service(insert)
            .service(update)
            .service(delete),
    );
}
