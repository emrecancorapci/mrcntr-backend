use actix_web::web;

use super::handlers::{many, one};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/experiences")
            .service(many)
            .service(one),
    );
}
