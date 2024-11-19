use crate::dto::Entry;
use crate::error::{JrnlIosError, JrnlIosResult};
use tauri_plugin_fs::FsExt;

mod context;
mod dto;
mod error;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_sign_in_with_apple::init())
        .plugin(tauri_plugin_google_signin::init())
        .setup(|app| {
            let scope = app.fs_scope();
            scope.allow_file("entries.mpk");

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            save_entry,
            get_entry,
            get_entries
        ])
        // context macro absolutely murders ide performance
        .run(context::context())
        .expect("error while running tauri application");
}

async fn load_entries() -> JrnlIosResult<Vec<Entry>> {
    let tokio_file_handle = tokio::fs::File::open("entries.mpk").await?;
    let std_file_handle = tokio_file_handle.into_std().await;

    rmp_serde::decode::from_read(std_file_handle).map_err(Into::into)
}

#[tauri::command]
async fn save_entry(entry: Entry) -> Result<(), JrnlIosError> {
    let mut entries = load_entries().await?;

    let existing_index = entries.iter().position(|e| e.id == entry.id);
    if existing_index {
        entries[existing_index] = entry;
    } else {
        entries.insert(0, entry);
    }

    entries.sort_by(|a, b| b.date.cmp(&a.date));

    let tokio_file_handle = tokio::fs::File::create("entries.mpk").await?;
    let mut std_file_handle = tokio_file_handle.into_std().await;

    rmp_serde::encode::write(&mut std_file_handle, &entries)
        .map_err(Into::into)
}

#[tauri::command]
async fn get_entry(id: String) -> JrnlIosResult<Option<Entry>> {
    Ok(
        load_entries().await?
            .into_iter()
            .find(|entry| entry.id == id)
    )
}

#[tauri::command]
async fn get_entries() -> JrnlIosResult<Vec<Entry>> {
    load_entries().await
}
