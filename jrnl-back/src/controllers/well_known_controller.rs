use crate::error::JrnlResult;
use crate::AppState;
use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;
use std::env;

pub fn well_known_controller() -> Router<AppState> {
    Router::new()
        .route("/apple-app-site-association", get(apple_deeplink_config))
}

#[derive(Serialize)]
struct AppleDeepLinkConfig {
    #[serde(rename = "applinks")]
    app_links: AppleDeepLinkDetails,
}

#[derive(Serialize)]
struct AppleDeepLinkDetails {
    details: Vec<AppleDeepLinkDetail>,
}

#[derive(Serialize)]
struct AppleDeepLinkDetail {
    #[serde(rename = "appIDs")]
    app_ids: Vec<String>,
    components: Vec<AppleDeepLinkComponent>,
}

#[derive(Serialize)]
struct AppleDeepLinkComponent {
    #[serde(rename = "/")]
    route: String,
    comment: String,
}

async fn apple_deeplink_config() -> JrnlResult<Json<AppleDeepLinkConfig>> {
    let app_id = format!(
        "{}.{}",
        env::var("APPLE_DEVELOPMENT_TEAM_ID").map_err(Into::<anyhow::Error>::into)?,
        env::var("APPLE_BUNDLE_ID").map_err(Into::<anyhow::Error>::into)?
    );

    let config = AppleDeepLinkConfig {
        app_links: AppleDeepLinkDetails {
            details: vec![
                AppleDeepLinkDetail {
                    app_ids: vec![app_id],
                    components: vec![
                        AppleDeepLinkComponent {
                            route: "/auth/apple/deeplink/*".to_string(),
                            comment: "Matches any URL whose path starts with /auth/apple/deeplink/".to_string(),
                        },
                    ],
                },
            ],
        },
    };

    Ok(Json(config))
}