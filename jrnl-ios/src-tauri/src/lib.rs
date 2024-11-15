use crate::dto::Entry;
use crate::error::{JrnlIosError, JrnlIosResult};
use tauri_plugin_fs::FsExt;

mod dto;
mod error;

// [todo] Refused to execute https://accounts.google.com/gsi/client as script because "X-Content-Type-Options: nosniff" was given and its Content-Type is not a script MIME type.

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
        .invoke_handler(tauri::generate_handler![load_entries, save_today_entry])
        .run(tauri::generate_context!())
        .expect("error while running jrnl ios app");
}



#[tauri::command]
async fn save_today_entry(entry: Entry) -> Result<(), JrnlIosError> {
    let tokio_file_handle = tokio::fs::File::create("entry-storage/today.mpk").await?;
    let mut std_file_handle = tokio_file_handle.into_std().await;

    rmp_serde::encode::write(&mut std_file_handle, &entry)
        .map_err(Into::into)
}

#[tauri::command]
async fn load_entries() -> JrnlIosResult<Vec<Entry>> {
    let tokio_file_handle = tokio::fs::File::open("entry-storage/entries.mpk").await?;
    let std_file_handle = tokio_file_handle.into_std().await;

    rmp_serde::from_read(std_file_handle)
        .map_err(Into::into)
}
