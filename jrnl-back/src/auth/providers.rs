use anyhow::Context;
use axum::http::header::AUTHORIZATION;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
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

#[derive(Deserialize, Debug)]
pub struct AppleCallbackPayload {
    /// A single-use authentication code that expires after five minutes. To learn how to validate this code to obtain user tokens, see Generate and validate tokens.
    /// <https://developer.apple.com/documentation/sign_in_with_apple/generate_and_validate_tokens>
    // pub code: String,
    /// A JSON web token (JWT) containing the user’s identification information. For more information, see Retrieve the user’s information from Apple ID servers.
    /// <https://developer.apple.com/documentation/sign_in_with_apple/sign_in_with_apple_rest_api/authenticating_users_with_sign_in_with_apple#3383773>
    pub id_token: String,
    /// An arbitrary string passed by the init function, representing the current state of the authorization request. This value is also used to mitigate cross-site request forgery attacks, by comparing against the state value contained in the authorization response.
    pub state: String,
    /// A JSON string containing the data requested in the scope property. The returned data is in the following format:
    #[serde(default, deserialize_with = "deserialize_maybe_user")]
    pub user: Option<AppleCallbackUser>,
}

fn deserialize_maybe_user<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<AppleCallbackUser>, D::Error> {
    let Ok(s) = String::deserialize(deserializer) else {
        return Ok(None);
    };

    serde_json::from_str(&s)
        .map(Some)
        .map_err(serde::de::Error::custom)
}

#[derive(Deserialize, Debug)]
pub struct AppleCallbackUser {
    // pub email: String,
    pub name: AppleCallbackUserName,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppleCallbackUserName {
    pub first_name: String,
    pub last_name: String,
}

#[derive(Deserialize, Debug)]
struct AppleIdTokenClaims {
    // The issuer registered claim identifies the principal that issues the identity token. Because Apple generates the token, the value is <https://appleid.apple.com>.
    iss: String, // https://appleid.apple.com
    /// The subject registered claim identifies the principal that’s the subject of the identity token. Because this token is for your app, the value is the unique identifier for the user.
    sub: String,
    /// The audience registered claim identifies the recipient of the identity token. Because the token is for your app, the value is the `client_id` from your developer account.
    aud: String, // fm.jrnl.jrnlapp
    ///The issued at registered claim indicates the time that Apple issues the identity token, in the number of seconds since the Unix epoch in UTC.
    // iat: i64,
    // The expiration time registered claim identifies the time that the identity token expires, in the number of seconds since the Unix epoch in UTC. The value must be greater than the current date and time when verifying the token.
    // exp: i64,
    /// A string for associating a client session with the identity token. This value mitigates replay attacks and is present only if you pass it in the authorization request.
    nonce: Option<String>,
    // A Boolean value that indicates whether the transaction is on a nonce-supported platform. If you send a nonce in the authorization request, but don’t see the nonce claim in the identity token, check this claim to determine how to proceed. If this claim returns true, treat nonce as mandatory and fail the transaction; otherwise, you can proceed treating the nonce as optional.
    // nonce_supported: bool,
    /// A string value that represents the user’s email address. The email address is either the user’s real email address or the proxy address, depending on their private email relay service. This value may be empty for Sign in with Apple at Work & School users. For example, younger students may not have an email address.
    email: String,
    // A string or Boolean value that indicates whether the service verifies the email. The value can either be a string ("true" or "false") or a Boolean (true or false). The system may not verify email addresses for Sign in with Apple at Work & School users, and this claim is "false" or false for those users.
    // email_verified: bool,
    // A string or Boolean value that indicates whether the email that the user shares is the proxy address. The value can either be a string ("true" or "false") or a Boolean (true or false).
    // is_private_email: bool,
    // An Integer value that indicates whether the user appears to be a real person. Use the value of this claim to mitigate fraud. The possible values are: 0 (or Unsupported), 1 (or Unknown), 2 (or LikelyReal). For more information, see ASUserDetectionStatus. This claim is present only in iOS 14 and later, macOS 11 and later, watchOS 7 and later, tvOS 14 and later. The claim isn’t present or supported for web-based apps.
    // real_user_status: i64,
    // A string value that represents the transfer identifier for migrating users to your team. This claim is present only during the 60-day transfer period after you transfer an app. For more information, see Bringing new apps and users into your team.
    // transfer_sub: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ApplePublicKeys {
    keys: Vec<ApplePublicKey>,
}

#[derive(Debug, Deserialize)]
struct ApplePublicKey {
    // kty: String,
    kid: String,
    // use_: String,
    // alg: String,
    n: String,
    e: String,
}

pub struct StrippedVerificationClaims {
    pub sub: String,
    pub email: String,
}

pub async fn verify_apple_id_token(
    id_token: &str,
    nonce: &str,
) -> anyhow::Result<StrippedVerificationClaims> {
    // To verify the identity token, your app server must:
    // Verify the JWS E256 signature using the server’s public key
    // Verify the nonce for the authentication
    // Verify that the iss field contains https://appleid.apple.com
    // Verify that the aud field is the developer’s client_id
    // Verify that the time is earlier than the exp value of the token
    let client = Client::new();

    let keys_response = client
        .get("https://appleid.apple.com/auth/keys")
        .send()
        .await?
        .json::<ApplePublicKeys>()
        .await?;

    let header = jsonwebtoken::decode_header(id_token)?;
    let kid = header.kid.context("missing kid")?;

    let key = keys_response
        .keys
        .into_iter()
        .find(|key| key.kid == kid)
        .context("no key found")?;

    let decoding_key = DecodingKey::from_rsa_components(&key.n, &key.e)?;
    let mut validation = Validation::new(Algorithm::RS256);

    validation.set_issuer(&["https://appleid.apple.com"]);
    validation.set_audience(&[&env::var("APPLE_CLIENT_ID")?]);

    let token_data =
        jsonwebtoken::decode::<AppleIdTokenClaims>(id_token, &decoding_key, &validation)?;

    if token_data.claims.nonce != Some(nonce.to_string()) {
        anyhow::bail!("nonce mismatch");
    }

    if token_data.claims.iss != "https://appleid.apple.com"
        || token_data.claims.aud != env::var("APPLE_CLIENT_ID")?
    {
        anyhow::bail!("bad token claim");
    }

    Ok(StrippedVerificationClaims {
        sub: token_data.claims.sub,
        email: token_data.claims.email,
    })
}
