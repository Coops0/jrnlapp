use anyhow::Context;
use axum::http::header::AUTHORIZATION;
use oauth2::{
    basic::BasicClient,
    AuthUrl,
    ClientId,
    ClientSecret,
    RedirectUrl,
    TokenUrl
};
use reqwest::Client;
use serde::Deserialize;
use std::env;

pub fn google_provider() -> anyhow::Result<BasicClient> {
    let base = env::var("FRONTEND_URL")?;

    let client = BasicClient::new(
        ClientId::new(env::var("GOOGLE_CLIENT_ID")?),
        Some(ClientSecret::new(env::var("GOOGLE_CLIENT_SECRET")?)),
        AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".parse()?)?,
        Some(TokenUrl::new(
            "https://www.googleapis.com/oauth2/v3/token".parse()?,
        )?),
    )
    .set_redirect_uri(RedirectUrl::new(format!("{base}/ac/google"))?);

    Ok(client)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GoogleResponse {
    names: Vec<Name>,
    email_addresses: Vec<EmailAddress>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct EmailAddress {
    metadata: Option<Metadata>,
    value: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Metadata {
    primary: Option<bool>,
    source_primary: Option<bool>,
}

impl Metadata {
    fn favorability(&self) -> u8 {
        let mut favorability = 0u8;

        if self.primary.unwrap_or(false) {
            favorability += 3;
        }

        if self.source_primary.unwrap_or(false) {
            favorability += 2;
        }

        favorability
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Name {
    metadata: Option<Metadata>,
    display_name: String,
}

pub async fn fetch_google_profile(token: &str) -> anyhow::Result<(String, String)> {
    let resp = Client::new()
        .get("https://people.googleapis.com/v1/people/me")
        .header(AUTHORIZATION, format!("Bearer {token}"))
        .header("Accept", "application/json")
        .query(&[("personFields", "names,emailAddresses")])
        .send()
        .await?
        .json::<GoogleResponse>()
        .await?;

    let (name, _) = resp
        .names
        .into_iter()
        .filter_map(|n| {
            if n.display_name.is_empty() {
                return None;
            }

            Some((
                n.display_name,
                n.metadata.as_ref().map_or(0, Metadata::favorability),
            ))
        })
        .max_by_key(|(_, favorability)| *favorability)
        .context("no name found")?;

    let (email, _) = resp
        .email_addresses
        .into_iter()
        .filter_map(|e| {
            if e.value.is_empty() {
                return None;
            }

            Some((
                e.value,
                e.metadata.as_ref().map_or(0, Metadata::favorability),
            ))
        })
        .max_by_key(|(_, favorability)| *favorability)
        .context("no email found")?;

    Ok((name, email))
}


pub fn apple_provider() -> anyhow::Result<BasicClient> {
    let base = env::var("FRONTEND_URL")?;

    let client = BasicClient::new(
        ClientId::new(env::var("APPLE_CLIENT_ID")?),
        Some(ClientSecret::new(env::var("APPLE_CLIENT_SECRET")?)),
        AuthUrl::new("https://appleid.apple.com/auth/authorize".parse()?)?,
        Some(TokenUrl::new("https://appleid.apple.com/auth/token".parse()?)?),
    )
    .set_redirect_uri(RedirectUrl::new(format!("{base}/ac/apple"))?);

    Ok(client)
}