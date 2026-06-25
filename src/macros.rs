#[macro_export]
macro_rules! resource {
    // 1. Public Entry Point: Initializes the state keys in a fixed order
    ( $($key:ident : $val:tt),* $(,)? ) => {
        $crate::resource! {
            @accumulate
            inputs: [ $($key : $val,)* ],
            scope: "",
            public: [],
            author: [],
            admin: []
        }
    };

    // 2. Parser Rules: Match an input key, update its specific position, and pass through the rest
    ( @accumulate inputs: [ scope: $path:expr, $($rest:tt)* ], scope: $_old:expr, public: $pub:tt, author: $auth:tt, admin: $adm:tt ) => {
        $crate::resource! { @accumulate inputs: [ $($rest)* ], scope: $path, public: $pub, author: $auth, admin: $adm }
    };
    ( @accumulate inputs: [ public: $val:tt, $($rest:tt)* ], scope: $path:expr, public: $_old:tt, author: $auth:tt, admin: $adm:tt ) => {
        $crate::resource! { @accumulate inputs: [ $($rest)* ], scope: $path, public: $val, author: $auth, admin: $adm }
    };
    ( @accumulate inputs: [ author: $val:tt, $($rest:tt)* ], scope: $path:expr, public: $pub:tt, author: $_old:tt, admin: $adm:tt ) => {
        $crate::resource! { @accumulate inputs: [ $($rest)* ], scope: $path, public: $pub, author: $val, admin: $adm }
    };
    ( @accumulate inputs: [ admin: $val:tt, $($rest:tt)* ], scope: $path:expr, public: $pub:tt, author: $auth:tt, admin: $_old:tt ) => {
        $crate::resource! { @accumulate inputs: [ $($rest)* ], scope: $path, public: $pub, author: $auth, admin: $val }
    };

    // 3. Final Evaluator: Code emits once the inputs array is completely empty `[]`
    (
        @accumulate
        inputs: [],
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
                    .wrap(actix_web::middleware::from_fn($crate::middlewares::auth::strict_to(vec![$crate::modules::auth::ROLE_AUTHOR, $crate::modules::auth::ROLE_ADMIN])))
                    .wrap(actix_web::middleware::from_fn($crate::middlewares::auth::auth_middleware))
                    $(.service($author_svc))*
            )
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
