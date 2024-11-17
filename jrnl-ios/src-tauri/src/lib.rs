use crate::dto::Entry;
use crate::error::{JrnlIosError, JrnlIosResult};
use chrono::Local;
use tauri_plugin_fs::FsExt;

mod dto;
mod error;
mod context;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_sign_in_with_apple::init())
        .setup(|app| {
            let scope = app.fs_scope();
            scope.allow_directory("entry-storage", true);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![load_entries, save_today_entry, load_today_entry])
        // context macro absolutely murders ide performance
        .run(context::context())
        .expect("error while running tauri application");
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
async fn load_today_entry() -> Option<Entry> {
    let tokio_file_handle = tokio::fs::File::open("entry-storage/today.mpk").await.ok()?;
    let std_file_handle = tokio_file_handle.into_std().await;

    rmp_serde::decode::from_read(std_file_handle).ok()
}

#[tauri::command]
async fn load_entries() -> JrnlIosResult<Vec<Entry>> {
    inner_load_entries().await
}