use crate::utils::now_unix;
use crate::CONFIG;
use eyre::Context;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

static ENCODING_KEY: Lazy<EncodingKey> =
    Lazy::new(|| EncodingKey::from_secret(CONFIG.jwt_secret.as_bytes()));

static DECODING_KEY: Lazy<DecodingKey> =
    Lazy::new(|| DecodingKey::from_secret(CONFIG.jwt_secret.as_bytes()));

static VALIDATION: Lazy<Validation> = Lazy::new(|| {
    let mut validation = Validation::new(Algorithm::HS512);
    validation.validate_exp = true;
    validation
});

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    /// Expiration. Unix timestamp.
    exp: u64,
    /// Issued at. Unix timestamp.
    iat: u64,
    /// Subject, i.e. stringified user UUID.
    sub: String,
}

pub fn generate_jwt(uuid: &Uuid) -> eyre::Result<String> {
    let now = now_unix();
    let claims = Claims {
        exp: now_unix() + (CONFIG.jwt_expiration_minutes * 60),
        iat: now,
        sub: uuid.to_string(),
    };
    let header = Header {
        alg: Algorithm::HS512,
        ..Default::default()
    };
    let token = encode(&header, &claims, &ENCODING_KEY)?;
    Ok(token)
}

pub fn validate_jwt(token: &str) -> eyre::Result<Uuid> {
    let data = decode::<Claims>(token, &DECODING_KEY, &VALIDATION)?;
    Uuid::from_str(&data.claims.sub).wrap_err("failed to parse uuid")
}
