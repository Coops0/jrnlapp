use oauth_axum::providers::google::GoogleProvider;
use oauth_axum::CustomProvider;
use std::env;

pub fn google_provider() -> anyhow::Result<CustomProvider> {
    let base = env::var("FRONTEND_URL")?;

    Ok(
        GoogleProvider::new(
            env::var("GOOGLE_CLIENT_ID")?,
            env::var("GOOGLE_CLIENT_SECRET")?,
            format!("{base}/ac/google"),
        )
    )
}