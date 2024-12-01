use crate::dto::Entry;
use crate::error::{JrnlIosError, JrnlIosResult};
use anyhow::Context;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use tauri_plugin_fs::FsExt;

mod context;
mod dto;
mod error;

fn resolve_entries_path(app_handle: &AppHandle) -> tauri::Result<PathBuf> {
    app_handle.path().app_cache_dir().map(|mut path| {
        path.push("entries.mpk");
        path
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_sign_in_with_apple::init())
        .plugin(tauri_plugin_google_signin::init())
        .append_invoke_initialization_script(
            r#"
                    try {
                        const theme = JSON.parse(localStorage.getItem('theme'));
                        if (theme) {
                            document.documentElement.setAttribute('data-theme', theme);
                        }
                    } catch (e) {
                        console.error('theme inject script: failed to parse theme', e);
                    }
            "#,
        )
        .setup(|app| {
            let scope = app.fs_scope();
            scope.allow_file(
                resolve_entries_path(app.handle()).expect("failed to resolve entries path"),
            );

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![save_entry, get_entry, get_entries])
        // context macro absolutely murders ide performance
        .run(context::context())
        .expect("error while running tauri application");
}

async fn load_entries(entries_path: &PathBuf) -> Option<Vec<Entry>> {
    let _ = tokio::fs::DirBuilder::new()
        .recursive(true)
        .create(
            entries_path
                .parent()
                .expect("failed to get entries path parent"),
        )
        .await;

    let tokio_file_handle = tokio::fs::OpenOptions::new()
        .read(true)
        .truncate(false)
        .create(true)
        .open(entries_path)
        .await
        .ok()?;

    let std_file_handle = tokio_file_handle.into_std().await;

    rmp_serde::decode::from_read(std_file_handle).ok()
}

#[tauri::command]
async fn save_entry(app_handle: AppHandle, entry: Entry) -> Result<(), JrnlIosError> {
    if entry.ephemeral {
        return Ok(());
    }

    let entries_path =
        resolve_entries_path(&app_handle).context("failed to resolve entries path")?;
    let mut entries = load_entries(&entries_path).await.unwrap_or_default();

    if let Some(existing_index) = entries.iter().position(|e| e.id == entry.id) {
        entries[existing_index] = entry;
    } else {
        entries.insert(0, entry);
    }

    entries.sort_by(|a, b| b.date.cmp(&a.date));

    let tokio_file_handle = tokio::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(entries_path)
        .await?;

    let mut std_file_handle = tokio_file_handle.into_std().await;

    rmp_serde::encode::write(&mut std_file_handle, &entries).map_err(Into::into)
}

#[tauri::command]
async fn get_entry(app_handle: AppHandle, id: String) -> JrnlIosResult<Option<Entry>> {
    Ok(
        load_entries(&resolve_entries_path(&app_handle).context("failed to resolve entries path")?)
            .await
            .unwrap_or_default()
            .into_iter()
            .find(|entry| entry.id == id),
    )
}

#[tauri::command]
async fn get_entries(app_handle: AppHandle) -> Vec<Entry> {
    load_entries(
        &resolve_entries_path(&app_handle)
            .context("failed to resolve entries path")
            .unwrap(),
    )
    .await
    .unwrap_or_default()
}
