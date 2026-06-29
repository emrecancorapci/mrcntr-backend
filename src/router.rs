use crate::{modules::projects::modules::*, modules::*, resource as r, scope as s};

pub fn routes(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(s! {
        scope: "/api",
        modules: [ s! {
            scope: "/v1",
            modules: [
                r! {
                    scope:  "/auth",
                    public: [auth::login, auth::register]
                },
                r! {
                    scope:  "/experiences",
                    public: [experiences::many, experiences::one],
                    admin:  [experiences::insert, experiences::update, experiences::delete]
                },
                r! {
                    scope:  "/tags",
                    public: [tags::many, tags::one],
                    admin:  [tags::insert, tags::update, tags::delete]
                },
                r! {
                    scope:  "/categories",
                    public: [categories::many, categories::one],
                    admin:  [categories::insert, categories::update, categories::delete]
                },
                r! {
                    scope:  "/projects",
                    public: [projects::many, projects::one],
                    admin:  [projects::insert, projects::update, projects::delete]
                },
                r! {
                    scope:  "/blogposts",
                    public: [blogposts::many, blogposts::one],
                    admin:  [blogposts::insert, blogposts::update, blogposts::delete]
                },
                r! {
                    scope:  "/comments",
                    public: [comments::insert, comments::update, comments::delete],
                    admin:  [comments::many, comments::one]
                },
                r! {
                    scope:  "/users",
                    admin:  [
                        users::many,
                        users::one,
                        users::insert,
                        users::update,
                        users::delete,
                    ]
                },
                r! {
                    scope:  "/project-blocks",
                    admin:  [
                        project_blocks::many,
                        project_blocks::one,
                        project_blocks::insert,
                        project_blocks::update,
                        project_blocks::delete,
                    ]
                },
                r! {
                    scope:  "/project-links",
                    admin:  [
                        project_links::many,
                        project_links::one,
                        project_links::insert,
                        project_links::update,
                        project_links::delete,
                    ]
                },
                r! {
                    scope:  "/project-statuses",
                    admin:  [
                        project_statuses::many,
                        project_statuses::one,
                        project_statuses::insert,
                        project_statuses::update,
                        project_statuses::delete,
                    ]
                },
                r! {
                    scope:  "/project-types",
                    admin:  [
                        project_types::many,
                        project_types::one,
                        project_types::insert,
                        project_types::update,
                        project_types::delete,
                    ]
                },
                r! {
                    scope:  "/project-ai_usages",
                    admin:  [
                        project_ai_usages::many,
                        project_ai_usages::one,
                        project_ai_usages::insert,
                        project_ai_usages::update,
                        project_ai_usages::delete,
                    ]
                },
                r! {
                    scope:  "/experiences-tags",
                    admin:  [
                        experiences_tags::insert_one,
                        experiences_tags::insert_many,
                        experiences_tags::replace_many_by_experience_id,
                        experiences_tags::delete
                    ]
                },
                r! {
                    scope:  "/projects-tags",
                    admin:  [
                        projects_tags::insert_one,
                        projects_tags::insert_many,
                        projects_tags::replace_many_by_project_id,
                        projects_tags::delete
                    ]
                },
            ]
        }]
    });
}
