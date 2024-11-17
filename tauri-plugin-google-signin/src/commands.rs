use tauri::{command, AppHandle, Runtime};

use crate::GoogleSigninExt;
use crate::Result;

#[command]
pub(crate) async fn request_signin<R: Runtime>(app: AppHandle<R>) -> Result<String> {
    app.google_signin().request_signin()
}

#[command]
pub(crate) async fn logout<R: Runtime>(app: AppHandle<R>) -> Result<String> {
    app.google_signin().logout()
}