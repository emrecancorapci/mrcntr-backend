use std::time::Duration;

use actix_limitation::Limiter;
use actix_session::SessionExt;
use actix_web::dev::ServiceRequest;

pub fn build_limiter() -> Limiter {
    let valkey_url = std::env::var("VALKEY_URL").expect("DATABASE_URL must be set");

    Limiter::builder(valkey_url)
        .key_by(|req: &ServiceRequest| {
            req.get_session()
                .get(&"session-id")
                .unwrap_or_else(|_| req.cookie(&"rate-api-id").map(|c| c.to_string()))
        })
        .limit(5000)
        .period(Duration::from_secs(3600)) // 60 minutes
        .build()
        .unwrap()
}
