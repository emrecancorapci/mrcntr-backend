use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use jsonwebtoken::{EncodingKey, Header};
use password_hash::phc::PasswordHash;

use crate::modules::auth::Claims;

pub fn generate_jwt(uuid: String) -> Result<String, Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let secret_key = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let key = EncodingKey::from_secret(secret_key.as_ref());

    let claims = Claims {
        uuid: uuid.clone(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    };

    let token_str = jsonwebtoken::encode(&Header::default(), &claims, &key)?;

    Ok(token_str)
}

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let argon2 = Argon2::default();

    let password_hash = argon2.hash_password(password.as_bytes())?.to_string();
    let parsed_hash = PasswordHash::new(&password_hash)?;

    Ok(parsed_hash.to_string())
}

pub fn verify_password(
    password: &str,
    password_hash: &str,
) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(password_hash)?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
