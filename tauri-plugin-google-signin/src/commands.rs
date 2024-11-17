use tauri::{command, AppHandle, Runtime};

use crate::GoogleSigninExt;
use crate::models::RequestSignInResponse;
use crate::Result;

#[command]
pub(crate) async fn request_signin<R: Runtime>(app: AppHandle<R>, nonce: String) -> Result<RequestSignInResponse> {
    app.google_signin().request_signin(nonce)
}

#[command]
pub(crate) async fn logout<R: Runtime>(app: AppHandle<R>) -> Result<()> {
    app.google_signin().logout()
}