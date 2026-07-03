pub mod project_ai_usages {
    mod handlers;
    mod models;
    pub mod repository;

    pub use handlers::*;
    pub use models::*;
}

pub mod project_blocks {
    mod handlers;
    mod models;
    pub mod repository;

    pub use handlers::*;
    pub use models::*;
}

pub mod project_links {
    mod handlers;
    mod models;
    pub mod repository;

    pub use handlers::*;
    pub use models::*;
}

pub mod project_statuses {
    mod handlers;
    mod models;
    pub mod repository;

    pub use handlers::*;
    pub use models::*;
}

pub mod project_types {
    mod handlers;
    mod models;
    pub mod repository;

    pub use handlers::*;
    pub use models::*;
}

pub mod projects_tags {
    mod handlers;
    mod models;
    pub mod repository;

    pub use handlers::*;
    pub use models::*;
}
