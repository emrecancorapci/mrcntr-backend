use crate::{modules::*, resource as r, scope as s};

pub fn routes(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(s! {
        scope: "/api",
        modules: [ s! {
            scope: "/v1",
            modules: [
                r! {
                    scope: "/auth",
                    public: [auth::login, auth::register]
                },
                r! {
                    scope: "/experiences",
                    public: [experiences::many, experiences::one],
                    admin: [experiences::insert, experiences::update, experiences::delete]
                },
                r! {
                    scope: "/tags",
                    public: [tags::many, tags::one],
                    admin: [tags::insert, tags::update, tags::delete]
                },
                r! {
                    scope: "/categories",
                    public: [categories::many, categories::one],
                    admin: [categories::insert, categories::update, categories::delete]
                },
                r! {
                    scope: "/projects",
                    public: [projects::many, projects::one],
                    admin: [projects::insert, projects::update, projects::delete]
                },
                r! {
                    scope: "/user",
                    admin: [
                        users::many,
                        users::one,
                        users::insert,
                        users::update,
                        users::delete,
                    ]
                },
                r! {
                    scope: "/experiences-tags",
                    admin: [
                        experiences_tags::insert_one,
                        experiences_tags::insert_many,
                        experiences_tags::replace_many_by_experience_id,
                        experiences_tags::delete
                    ]
                },
            ]
        }]
    });
}
