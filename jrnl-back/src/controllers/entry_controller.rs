use crate::{
    error::{DatabaseError, JrnlError, JrnlResult, JsonExtractor},
    schemas::{active_entry::ActiveEntry, entry::DecryptedEntry, user::User},
    services::entry_service::{EntryService, StrippedEntry},
    web::cursor::{Cursor, CursorPaginatedResponse, CursorParams},
    AppState,
};
use aes_gcm::{Aes256Gcm, Key};
use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use chrono::{Duration, NaiveDate, Utc};
use serde::Deserialize;
use tokio::task::spawn_blocking;
use tracing::error;
use uuid::Uuid;

pub fn entries_controller() -> Router<AppState> {
    Router::new()
        .route("/", get(get_trimmed_entries_paginated).put(put_local_mobile_entries))
        .route("/:id", get(get_entry))
        .route("/today", get(get_today_entry).put(update_today_entry))
}

async fn encrypt_active_entries_except_today(
    user: &User,
    entry_service: &EntryService,
    master_key: Key<Aes256Gcm>,
) -> anyhow::Result<()> {
    let (mut transaction, entries) = entry_service
        .create_entry_migration_transaction_without_today(user)
        .await?;

    if entries.is_empty() {
        return Ok(());
    }

    let encrypted_entries = match spawn_blocking(move || -> anyhow::Result<_> {
        let encrypted_entries = entries
            .into_iter()
            .filter(|entry| !entry.ephemeral)
            .map(|entry| ActiveEntry::encrypt(&entry, &master_key))
            .collect::<anyhow::Result<Vec<_>>>()
            .map_err(JrnlError::EntryEncryptionFailed)?;

        Ok(encrypted_entries)
    }).await? {
        Ok(entries) => entries,
        Err(e) => {
            error!("Failed to encrypt entries: {:?}", e);
            transaction.rollback().await?;
            return Err(e);
        }
    };

    for entry in encrypted_entries {
        let Err(why) = EntryService::create_encrypted_entry_query(&entry)
            .execute(&mut *transaction)
            .await else { continue; };

        error!("Failed to insert encrypted entry: {:?}", why);
        transaction.rollback().await?;
        return Err(why.into());
    }

    transaction.commit().await.map_err(Into::into)
}

async fn get_trimmed_entries_paginated(
    user: User,
    Query(params): Query<CursorParams>,
    entry_service: EntryService,
    State(AppState { master_key, .. }): State<AppState>,
) -> JrnlResult<Json<CursorPaginatedResponse<StrippedEntry>>> {
    let limit = params.limit.unwrap_or(20).clamp(1, 100);

    let cursor = params.cursor.unwrap_or_default();
    encrypt_active_entries_except_today(&user, &entry_service, master_key).await?;

    let mut entries = entry_service
        .get_paginated_trimmed_entries(&user, &cursor, i64::from(limit))
        .await
        .map_err(DatabaseError)?;

    let has_more = entries.len() > limit as usize;
    if has_more {
        entries.pop();
    }

    let next_cursor = match (has_more, entries.last()) {
        (true, Some(last_entry)) => Some(Cursor {
            id: last_entry.id,
            date: last_entry.date,
        }),
        _ => None,
    };

    Ok(Json(CursorPaginatedResponse { items: entries, next_cursor, has_more }))
}

async fn get_entry(
    user: User,
    Path(id): Path<Uuid>,
    entry_service: EntryService,
    State(AppState { master_key, .. }): State<AppState>,
) -> JrnlResult<Json<Option<DecryptedEntry>>> {
    let Some(encrypted_entry) = entry_service.get_entry_maybe(&user, &id).await? else {
        return Ok(Json(None));
    };

    let decrypted_entry = spawn_blocking(move || encrypted_entry.decrypt(&master_key))
        .await
        .map_err(Into::<anyhow::Error>::into)
        .map_err(JrnlError::EntryDecryptionFailed)??;

    Ok(Json(Some(decrypted_entry)))
}

async fn get_today_entry(
    user: User,
    entry_service: EntryService,
) -> JrnlResult<Json<Option<ActiveEntry>>> {
    entry_service
        .get_user_daily_entry_maybe(&user)
        .await
        .map(Json)
        .map_err(Into::into)
}

#[derive(Deserialize)]
struct UpdateEntryPayload {
    emotion_scale: f32,
    #[serde(default, deserialize_with = "sanitize_html_string")]
    text: Option<String>,
    #[serde(default)]
    ephemeral: bool
}

#[allow(clippy::unnecessary_wraps)]
fn sanitize_html_string<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<String>, D::Error> {
    let Ok(s) = String::deserialize(deserializer) else {
        return Ok(None);
    };

    let trimmed = s.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }

    let cleaned = ammonia::clean(trimmed);
    Ok(Some(cleaned))
}

async fn update_today_entry(
    user: User,
    entry_service: EntryService,
    JsonExtractor(payload): JsonExtractor<UpdateEntryPayload>,
) -> JrnlResult<Json<ActiveEntry>> {
    entry_service
        .update_or_create_daily_entry(&user, payload.emotion_scale, payload.text, payload.ephemeral)
        .await
        .map(Json)
        .map_err(Into::into)
}

#[derive(Deserialize)]
struct MobilePastEntry {
    date: NaiveDate,
    emotion_scale: f32,
    #[serde(default, deserialize_with = "sanitize_html_string")]
    text: Option<String>,
}

async fn put_local_mobile_entries(
    user: User,
    entry_service: EntryService,
    State(AppState { master_key, .. }): State<AppState>,
    JsonExtractor(entries): JsonExtractor<Vec<MobilePastEntry>>,
) -> JrnlResult<()> {
    let today = user.current_date_by_timezone();

    let entries = entries
        .into_iter()
        .filter(|entry| entry.date < today)
        .map(|entry| ActiveEntry {
            id: Uuid::new_v4(),
            author: user.id,
            date: entry.date,
            emotion_scale: entry.emotion_scale,
            text: entry.text,
            // this should never get hit
            expiry: Utc::now() + Duration::days(30),
            ephemeral: false,
        })
        .collect::<Vec<_>>();

    if entries.is_empty() {
        return Ok(());
    }

    if entries.len() > 365 {
        return Err(JrnlError::TooManyEntries);
    }

    entry_service.insert_many_entries(entries, master_key).await
}