use actix_web::{middleware::from_fn, web};

use crate::{
    middlewares::auth::{auth_middleware, strict_to},
    modules::*,
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api").service(
            //
            // V1
            //
            web::scope("/v1")
                //
                // PUBLIC
                //
                .service(
                    web::scope("/auth")
                        .service(auth::login)
                        .service(auth::register),
                )
                //
                // PARTLY PROTECTED
                //
                .service(
                    web::scope("/experiences")
                        .service(experiences::many)
                        .service(experiences::one)
                        .service(
                            web::scope("")
                                .wrap(from_fn(strict_to(vec!["admin"])))
                                .wrap(from_fn(auth_middleware))
                                .service(experiences::insert)
                                .service(experiences::update)
                                .service(experiences::delete),
                        ),
                )
                .service(
                    web::scope("/tags")
                        .service(tags::many)
                        .service(tags::one)
                        .service(
                            web::scope("")
                                .wrap(from_fn(strict_to(vec!["admin"])))
                                .wrap(from_fn(auth_middleware))
                                .service(tags::insert)
                                .service(tags::update)
                                .service(tags::delete),
                        ),
                )
                .service(
                    web::scope("/categories")
                        .service(categories::many)
                        .service(categories::one)
                        .service(
                            web::scope("")
                                .wrap(from_fn(strict_to(vec!["admin"])))
                                .wrap(from_fn(auth_middleware))
                                .service(categories::insert)
                                .service(categories::update)
                                .service(categories::delete),
                        ),
                )
                .service(
                    web::scope("/projects")
                        .service(projects::many)
                        .service(projects::one)
                        .service(
                            web::scope("")
                                .wrap(from_fn(strict_to(vec!["admin"])))
                                .wrap(from_fn(auth_middleware))
                                .service(projects::insert)
                                .service(projects::update)
                                .service(projects::delete),
                        ),
                )
                //
                // FULLY PROTECTED
                //
                .service(
                    web::scope("/user")
                        .wrap(from_fn(strict_to(vec!["admin"])))
                        .wrap(from_fn(auth_middleware))
                        .service(users::many)
                        .service(users::one)
                        .service(users::insert)
                        .service(users::update)
                        .service(users::delete),
                )
                .service(
                    web::scope("/experiences-tags")
                        .wrap(from_fn(strict_to(vec!["admin"])))
                        .wrap(from_fn(auth_middleware))
                        .service(experiences_tags::insert_one)
                        .service(experiences_tags::insert_many)
                        .service(experiences_tags::replace_many_by_experience_id)
                        .service(experiences_tags::delete),
                ),
        ),
    );
}
