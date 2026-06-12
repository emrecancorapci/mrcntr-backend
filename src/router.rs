use actix_web::{middleware::from_fn, web};

use crate::{
    middlewares::auth::auth_middleware,
    modules::{auth, categories, experiences, experiences_tags, tags, users},
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api").service(
            web::scope("/v1")
                // Experiences - Public
                .service(
                    web::scope("/experiences")
                        .service(experiences::many)
                        .service(experiences::one),
                )
                // Experiences - Protected
                .service(
                    web::scope("/experiences")
                        .wrap(from_fn(auth_middleware))
                        .service(experiences::insert)
                        .service(experiences::update)
                        .service(experiences::delete),
                )
                // Tags - Public
                .service(web::scope("/tags").service(tags::many).service(tags::one))
                // Tags - Protected
                .service(
                    web::scope("/tags")
                        .wrap(from_fn(auth_middleware))
                        .service(tags::insert)
                        .service(tags::update)
                        .service(tags::delete),
                )
                // Experiences-Tags - Protected
                .service(
                    web::scope("/experiences-tags")
                        .wrap(from_fn(auth_middleware))
                        .service(experiences_tags::insert_one)
                        .service(experiences_tags::insert_many)
                        .service(experiences_tags::replace_many_by_experience_id)
                        .service(experiences_tags::delete),
                )
                // Categories - Public
                .service(
                    web::scope("/categories")
                        .service(categories::many)
                        .service(categories::one),
                )
                // Categories - Protected
                .service(
                    web::scope("/categories")
                        .wrap(from_fn(auth_middleware))
                        .service(categories::insert)
                        .service(categories::update)
                        .service(categories::delete),
                )
                // User - Protected
                .service(
                    web::scope("/user")
                        .wrap(from_fn(auth_middleware))
                        .service(users::many)
                        .service(users::one)
                        .service(users::insert)
                        .service(users::update)
                        .service(users::delete),
                )
                // Auth - Public
                .service(
                    web::scope("/auth")
                        .service(auth::login)
                        .service(auth::register),
                ),
        ),
    );
}
