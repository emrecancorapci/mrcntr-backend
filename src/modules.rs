pub mod auth;
pub mod categories;
pub mod experiences;
pub mod projects;
pub mod roles;
pub mod tags;
pub mod users;
pub mod comments;
pub mod blogposts;

pub mod project_blocks {
    mod handlers;
    mod models;
    pub mod repository;

    pub use handlers::*;
    pub use models::*;
}