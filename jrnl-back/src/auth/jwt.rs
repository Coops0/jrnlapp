use chrono::Utc;
use jsonwebtoken::{encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::{
    cell::LazyCell,
    env
};
use uuid::Uuid;

const JWT_KEYS: LazyCell<(EncodingKey, DecodingKey)> = LazyCell::new(|| {
    let key_str = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let key = key_str.as_bytes();

    (EncodingKey::from_secret(key), DecodingKey::from_secret(key))
});

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
}

pub fn encode_user_jwt(uid: Uuid) -> anyhow::Result<String> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::days(30))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: uid,
        exp: usize::try_from(expiration)?,
    };

    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &JWT_KEYS.0).map_err(Into::into)
}

pub fn decode_user_jwt(token: &str) -> anyhow::Result<Claims> {
    jsonwebtoken::decode::<Claims>(token, &JWT_KEYS.1, &Validation::new(Algorithm::HS512))
        .map(|data| data.claims)
        .map_err(Into::into)
}
