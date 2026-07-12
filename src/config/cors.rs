use std::env;

use actix_cors::Cors;

pub fn app_cors() -> Cors {
    let allowed_str = env::var("ALLOWED_ORIGINS").expect("ALLOWED_ORIGINS must be set");
    let allowed_urls: Vec<String> = allowed_str
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    Cors::default()
        .allowed_origin_fn(move |origin, _req| {
            if let Ok(origin_str) = origin.to_str() {
                allowed_urls.iter().any(|allowed_url| {
                    origin_str == allowed_url || origin_str.starts_with(allowed_url)
                })
            } else {
                false
            }
        })
        .allow_any_method()
        .allow_any_header()
}
