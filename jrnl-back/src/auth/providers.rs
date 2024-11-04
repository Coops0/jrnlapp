use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl};
use std::env;

pub fn google_provider() -> anyhow::Result<BasicClient> {
    let base = env::var("FRONTEND_URL")?;
    let client = BasicClient::new(
        ClientId::new(env::var("GOOGLE_CLIENT_ID")?),
        Some(ClientSecret::new(env::var("GOOGLE_CLIENT_SECRET")?)),
        AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".parse()?)?,
        None,
    ).set_redirect_uri(RedirectUrl::new(format!("{base}/ac/google"))?);
    
    Ok(client)
}
