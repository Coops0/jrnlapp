use crate::dto::Entry;
use crate::error::{JrnlIosError, JrnlIosResult};
use chrono::Local;
use tauri_plugin_fs::FsExt;
use tauri_plugin_http::reqwest;

mod dto;
mod error;

// Refused to execute https://accounts.google.com/gsi/client as script because "X-Content-Type-Options: nosniff" was given and its Content-Type is not a script MIME type.

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let scope = app.fs_scope();
            scope.allow_directory("entry-storage", true);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![load_entries, save_today_entry, proxy_google_script])
        .run(tauri::generate_context!())
        .expect("error while running jrnl ios app");
}

async fn handle_today_entry() -> JrnlIosResult<()> {
    let tokio_file_handle = tokio::fs::File::open("entry-storage/today.mpk").await?;
    let std_file_handle = tokio_file_handle.into_std().await;

    let entry = rmp_serde::decode::from_read::<_, Entry>(std_file_handle)?;
    let today = Local::now().date_naive();

    // same day, we're okay to overwrite
    if entry.date.eq(&today) {
        return Ok(());
    }

    let mut existing_entries = inner_load_entries().await?;
    existing_entries.insert(0, entry);

    let tokio_file_handle = tokio::fs::File::create("entry-storage/entries.mpk").await?;
    let mut std_file_handle = tokio_file_handle.into_std().await;

    rmp_serde::encode::write(&mut std_file_handle, &existing_entries)
        .map_err(Into::into)
}

async fn inner_load_entries() -> JrnlIosResult<Vec<Entry>> {
    let tokio_file_handle = tokio::fs::File::open("entry-storage/entries.mpk").await?;
    let std_file_handle = tokio_file_handle.into_std().await;

    rmp_serde::decode::from_read(std_file_handle)
        .map_err(Into::into)
}

#[tauri::command]
async fn save_today_entry(entry: Entry) -> Result<(), JrnlIosError> {
    // if current cache is from yesterday, save to entries list
    let _ = handle_today_entry().await;

    let tokio_file_handle = tokio::fs::File::create("entry-storage/today.mpk").await?;
    let mut std_file_handle = tokio_file_handle.into_std().await;

    rmp_serde::encode::write(&mut std_file_handle, &entry)
        .map_err(Into::into)
}

#[tauri::command]
async fn load_entries() -> JrnlIosResult<Vec<Entry>> {
    inner_load_entries().await
}


#[tauri::command]
async fn proxy_google_script() -> JrnlIosResult<String> {
    let response = reqwest::get("https://accounts.google.com/gsi/client").await?;
    response.text()
        .await
        .map_err(Into::into)
}