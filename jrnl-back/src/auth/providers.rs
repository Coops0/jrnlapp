use anyhow::Context;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use reqwest::get;
use serde::{
    de::DeserializeOwned,
    Deserialize,
};
use std::env;
use uuid::Uuid;

#[derive(Deserialize)]
struct JwtPublicKeys {
    keys: Vec<JWKPublicKey>,
}

#[derive(Deserialize)]
struct JWKPublicKey {
    kid: String,
    n: String,
    e: String,
}

async fn verify_token<Claims: DeserializeOwned>(url: &str, token: &str, issuer: &[&str], audience: &[&str]) -> anyhow::Result<Claims> {
    // FIXME: cache these keys and use their headers to determine when to refresh
    let keys_response = get(url)
        .await?
        .json::<JwtPublicKeys>()
        .await?;

    let header = jsonwebtoken::decode_header(token)?;
    let kid = header.kid.context("missing kid")?;

    let key = keys_response
        .keys
        .into_iter()
        .find(|key| key.kid == kid)
        .context("no key found")?;

    let decoding_key = DecodingKey::from_rsa_components(&key.n, &key.e)?;
    let mut validation = Validation::new(Algorithm::RS256);

    validation.set_issuer(issuer);
    validation.set_audience(audience);

    jsonwebtoken::decode::<Claims>(token, &decoding_key, &validation)
        .map(|data| data.claims)
        .map_err(Into::into)
}

#[derive(Deserialize)]
pub struct GoogleIdTokenClaims {
    sub: String,
    name: Option<String>,
    given_name: Option<String>,
    nonce: String,
}

pub struct StrippedGoogleVerificationClaims {
    pub sub: String,
    pub name: Option<String>,
}

pub async fn verify_google_credential(credential: &str, nonce: &Uuid) -> anyhow::Result<StrippedGoogleVerificationClaims> {
    let claims = verify_token::<GoogleIdTokenClaims>(
        "https://www.googleapis.com/oauth2/v3/certs",
        credential,
        &["https://accounts.google.com", "accounts.google.com"],
        &[&env::var("GOOGLE_CLIENT_ID")?],
    ).await?;

    if claims.nonce != nonce.to_string() {
        anyhow::bail!("nonce mismatch");
    }

    let name = claims.name.or(claims.given_name);
    Ok(StrippedGoogleVerificationClaims { sub: claims.sub, name })
}

#[derive(Deserialize)]
pub struct AppleCallbackPayload {
    pub authorization: AppleCallbackAuthorization,
    pub user: Option<AppleCallbackUser>,
}

#[derive(Deserialize)]
pub struct AppleCallbackAuthorization {
    pub id_token: String,
    pub state: Uuid,
}

#[derive(Deserialize)]
pub struct AppleCallbackUser {
    pub name: AppleCallbackUserName,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppleCallbackUserName {
    pub first_name: String,
}

#[derive(Deserialize)]
struct AppleIdTokenClaims {
    sub: String,
    nonce: Option<String>,
}

pub async fn verify_apple_id_token(id_token: &str, nonce: &Uuid) -> anyhow::Result<String> {
    let claims = verify_token::<AppleIdTokenClaims>(
        "https://appleid.apple.com/auth/keys",
        id_token,
        &["https://appleid.apple.com"],
        &[&env::var("APPLE_CLIENT_ID")?],
    ).await?;

    if claims.nonce != Some(nonce.to_string()) {
        anyhow::bail!("nonce mismatch");
    }

    Ok(claims.sub)
}
