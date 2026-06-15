#[macro_export]
macro_rules! resource {
    (
        scope: $path:expr,
        public: [ $($pub_svc:expr),* $(,)? ],
        author: [ $($author_svc:expr),* $(,)? ],
        admin: [ $($admin_svc:expr),* $(,)? ]
    ) => {
        actix_web::web::scope($path)
            $(.service($pub_svc))*
            .service(
                actix_web::web::scope("")
                    .wrap(actix_web::middleware::from_fn($crate::middlewares::auth::strict_to(vec![$crate::modules::auth::ROLE_ADMIN])))
                    .wrap(actix_web::middleware::from_fn($crate::middlewares::auth::auth_middleware))
                    $(.service($admin_svc))*
            )
            .service(
                actix_web::web::scope("")
                    .wrap(actix_web::middleware::from_fn($crate::middlewares::auth::strict_to(vec![$crate::modules::auth::ROLE_AUTHOR])))
                    .wrap(actix_web::middleware::from_fn($crate::middlewares::auth::auth_middleware))
                    $(.service($author_svc))*
            )
    };

    (
        scope: $path:expr,
        public: [ $($pub_svc:expr),* $(,)? ],
        admin: [ $($admin_svc:expr),* $(,)? ]
    ) => {
        $crate::resource! {
            scope: $path,
            public: [ $($pub_svc),* ],
            author: [],
            admin: [ $($admin_svc),* ]
        }
    };

    (
        scope: $path:expr,
        public: [ $($pub_svc:expr),* $(,)? ]
    ) => {
        $crate::resource! {
            scope: $path,
            public: [ $($pub_svc),* ],
            admin: []
        }
    };

    (
        scope: $path:expr,
        admin: [ $($admin_svc:expr),* $(,)? ]
    ) => {
        $crate::resource! {
            scope: $path,
            public: [],
            admin: [ $($admin_svc),* ]
        }
    };
}

#[macro_export]
macro_rules! scope {
    (
        scope: $path:expr,
        modules: [ $($module_svc:expr),* $(,)? ]
    ) => {
        actix_web::web::scope($path)
            $(.service($module_svc))*
    };
}
