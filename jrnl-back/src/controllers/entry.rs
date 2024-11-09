use crate::schemas::active_entry::ActiveEntry;
use crate::schemas::entry::DecryptedEntry;
use crate::{
    error::{DatabaseError, JrnlResult, JsonExtractor},
    schemas::entry::EncryptedEntry,
    schemas::user::User,
    web::cursor::{Cursor, CursorPaginatedResponse, CursorParams},
    AppState,
};
use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json,
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use tokio::task::spawn_blocking;
use tracing::error;
use uuid::Uuid;

pub fn entries_controller() -> Router<AppState> {
    Router::new()
        .route("/", get(get_trimmed_entries_paginated))
        .route("/:id", get(get_entry))
        .route("/average", get(get_overall_average))
        .route("/today", get(get_today_entry).put(update_today_entry))
}

#[derive(Serialize, FromRow)]
struct StrippedEntry {
    emotion_scale: f32,
    date: chrono::NaiveDate,
    id: Uuid,
}

async fn encrypt_active_entries(user: User, pool: &PgPool) -> anyhow::Result<()> {
    let today_date = user.current_date_by_timezone();

    let mut transaction = pool.begin().await?;

    let entries = sqlx::query_as::<_, ActiveEntry>(
        // language=postgresql
        "DELETE FROM active_entries WHERE author = $1 AND NOT date = $2 RETURNING *",
    )
        .bind(user.id)
        .bind(today_date)
        .fetch_all(&mut *transaction)
        .await?;

    if entries.is_empty() {
        return Ok(());
    }

    let encrypted_entries = match spawn_blocking(move || -> anyhow::Result<_> {
        let encrypted_entries = entries.into_iter()
            .map(|entry| entry.encrypt(/* todo master encryption key */0))
            .collect::<anyhow::Result<Vec<_>>>()?;

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
        if let Err(why) = sqlx::query(
            // language=postgresql
            "
            INSERT INTO entries (id, author, date, emotion_scale, encrypted_content, content_key, nonce)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ",
        )
            .bind(entry.id)
            .bind(entry.author)
            .bind(entry.date)
            .bind(entry.emotion_scale)
            .bind(entry.encrypted_content)
            .bind(entry.content_key)
            .bind(entry.nonce)
            .execute(&mut *transaction)
            .await {
            error!("Failed to insert encrypted entry: {:?}", why);
            transaction.rollback().await?;
            return Err(why.into());
        }
    }

    transaction.commit()
        .await
        .map_err(Into::into)
}

async fn get_trimmed_entries_paginated(
    user: User,
    Query(params): Query<CursorParams>,
    State(AppState { pool }): State<AppState>,
) -> JrnlResult<Json<CursorPaginatedResponse<StrippedEntry>>> {
    let limit = params.limit.unwrap_or(20).clamp(1, 100);
    let limit_plus_one = i64::from(limit) + 1;

    let cursor = params.cursor.unwrap_or_default();

    let mut entries = sqlx::query_as::<_, StrippedEntry>(
        // language=postgresql
        "
            SELECT emotion_scale, date, id FROM entries
            WHERE entries.author = $1
            AND (date, id) < ($2, $3)
            ORDER BY date DESC, id DESC
            LIMIT $4
            ",
    )
        .bind(user.id)
        .bind(cursor.date)
        .bind(cursor.id)
        .bind(limit_plus_one)
        .fetch_all(&pool)
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

    Ok(Json(CursorPaginatedResponse {
        items: entries,
        next_cursor,
        has_more,
    }))
}

async fn get_entry(
    user: User,
    Path(id): Path<Uuid>,
    State(AppState { pool }): State<AppState>,
) -> JrnlResult<Json<Option<DecryptedEntry>>> {
    let Some(encrypted_entry) = sqlx::query_as::<_, EncryptedEntry>(
        // language=postgresql
        "SELECT * FROM entries WHERE author = $1 AND id = $2 LIMIT 1",
    )
        .bind(user.id)
        .bind(id)
        .fetch_optional(&pool)
        .await? else {
        return Ok(Json(None));
    };

    let decrypted_entry = encrypted_entry.decrypt(/* todo master encryption key */0)?;
    Ok(Json(Some(decrypted_entry)))
}

async fn get_overall_average(
    user: User,
    State(AppState { pool, .. }): State<AppState>,
) -> JrnlResult<Json<f64>> {
    sqlx::query_scalar(
        // language=postgresql
        "SELECT AVG(emotion_scale) FROM entries WHERE author = $1",
    )
        .bind(user.id)
        .fetch_one(&pool)
        .await
        .map(Json)
        .map_err(Into::into)
}

async fn get_today_entry(
    user: User,
    State(AppState { pool }): State<AppState>,
) -> JrnlResult<Json<Option<ActiveEntry>>> {
    sqlx::query_as::<_, ActiveEntry>(
        // language=postgresql
        "SELECT * FROM active_entries WHERE author = $1 AND date = $2 LIMIT 1",
    )
        .bind(user.id)
        .bind(user.current_date_by_timezone())
        .fetch_optional(&pool)
        .await
        .map(Json)
        .map_err(Into::into)
}

#[derive(Deserialize)]
struct UpdateEntryPayload {
    emotion_scale: f32,
    #[serde(default, deserialize_with = "sanitize_html_string")]
    text: Option<String>,
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
    State(AppState { pool }): State<AppState>,
    JsonExtractor(payload): JsonExtractor<UpdateEntryPayload>,
) -> JrnlResult<Json<EncryptedEntry>> {
    sqlx::query_as::<_, EncryptedEntry>(
        // language=postgresql
        "
            INSERT INTO active_entries (author, date, emotion_scale, text) VALUES ($1, $2, $3, $4)
            ON CONFLICT (author, date) 
            DO UPDATE SET emotion_scale = $3, text = $4 
            RETURNING *
        ",
    )
        .bind(user.id)
        .bind(user.current_date_by_timezone())
        .bind(payload.emotion_scale)
        .bind(payload.text)
        .fetch_one(&pool)
        .await
        .map(Json)
        .map_err(Into::into)
}
