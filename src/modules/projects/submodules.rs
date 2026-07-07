pub mod blocks {
    pub mod handlers;

    pub use handlers::*;
}

pub mod links {
    pub mod handlers;

    pub use handlers::*;
}

pub mod tags {
    mod handlers;
    mod models;
    pub mod repository;

    pub use handlers::*;
    pub use models::*;
}
