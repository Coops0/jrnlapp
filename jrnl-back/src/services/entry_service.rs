use crate::error::JrnlResult;
use crate::{
    impl_service,
    schemas::{
        active_entry::ActiveEntry,
        entry::EncryptedEntry,
        user::User,
    },
    web::cursor::Cursor,
};
use aes_gcm::{Aes256Gcm, Key};
use chrono::{NaiveDate, Timelike};
use serde::Serialize;
use sqlx::{
    postgres::PgArguments,
    query::Query,
    Error,
    FromRow,
    PgPool,
    Postgres,
    Transaction,
};
use std::time::Duration;
use tokio::{
    task::spawn_blocking,
    time::interval,
};
use uuid::Uuid;

pub struct EntryService(PgPool);

impl_service!(EntryService);

#[derive(Serialize, FromRow)]
pub struct StrippedEntry {
    pub emotion_scale: f32,
    pub date: NaiveDate,
    pub id: Uuid,
}

#[derive(FromRow)]
pub struct DayDataRow {
    pub emotion_scale: f32,
    pub date: NaiveDate,
}

impl EntryService {
    pub async fn create_entry_migration_transaction_without_today(&self, user: &User) -> Result<(Transaction<'_, Postgres>, Vec<ActiveEntry>), Error> {
        let mut transaction = self.0.begin().await?;
        let entries = sqlx::query_as::<_, ActiveEntry>(
            // language=postgresql
            "DELETE FROM active_entries WHERE author = $1 AND expiry < timezone('utc', now()) RETURNING *",
        )
            .bind(user.id)
            .fetch_all(&mut *transaction)
            .await?;

        Ok((transaction, entries))
    }

    pub fn create_encrypted_entry_query(entry: &EncryptedEntry) -> Query<Postgres, PgArguments> {
        sqlx::query(
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
            .bind(&entry.encrypted_content)
            .bind(&entry.content_key)
            .bind(&entry.nonce)
    }

    pub async fn get_paginated_trimmed_entries(
        &self,
        user: &User,
        cursor: &Cursor,
        limit: i64,
    ) -> Result<Vec<StrippedEntry>, Error> {
        sqlx::query_as(
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
            .bind(limit + 1)
            .fetch_all(&self.0)
            .await
    }

    pub async fn get_entry_maybe(&self, user: &User, id: &Uuid) -> Result<Option<EncryptedEntry>, Error> {
        sqlx::query_as(
            // language=postgresql
            "SELECT * FROM entries WHERE author = $1 AND id = $2 LIMIT 1",
        )
            .bind(user.id)
            .bind(id)
            .fetch_optional(&self.0)
            .await
    }

    pub async fn get_user_daily_entry_maybe(&self, user: &User) -> Result<Option<ActiveEntry>, Error> {
        sqlx::query_as(
            // language=postgresql
            "SELECT * FROM active_entries WHERE author = $1 AND date = $2 LIMIT 1",
        )
            .bind(user.id)
            .bind(user.current_date_by_timezone())
            .fetch_optional(&self.0)
            .await
    }

    pub async fn update_or_create_daily_entry(
        &self,
        user: &User,
        emotion_scale: f32,
        text: Option<String>,
    ) -> Result<ActiveEntry, Error> {
        let expiry = user.current_date_time_by_timezone() + chrono::Duration::days(1);
        let expiry_midnight = expiry.with_hour(0)
            .and_then(|d| d.with_minute(0))
            .and_then(|d| d.with_second(0))
            .unwrap_or(expiry)
            .to_utc();

        sqlx::query_as(
            // language=postgresql
            "
                INSERT INTO active_entries (author, date, emotion_scale, text, expiry) VALUES ($1, $2, $3, $4, $5)
                ON CONFLICT (author, date)
                DO UPDATE SET emotion_scale = $3, text = $4
                RETURNING *
            ",
        )
            .bind(user.id)
            .bind(user.current_date_by_timezone())
            .bind(emotion_scale)
            .bind(text)
            .bind(expiry_midnight)
            .fetch_one(&self.0)
            .await
    }

    pub async fn get_multiple_users_entries_between_dates(
        &self,
        group_member_ids: &[Uuid],
        start_date: &NaiveDate,
        before_date: &NaiveDate,
    ) -> Result<Vec<DayDataRow>, Error> {
        sqlx::query_as(
            // language=postgresql
            "
                SELECT date, emotion_scale FROM entries
                WHERE author = ANY($1)
                AND date >= $2
                AND date <= $3
                ORDER BY date DESC
                LIMIT 500
        ",
        )
            .bind(group_member_ids)
            .bind(start_date)
            .bind(before_date)
            .fetch_all(&self.0)
            .await
    }

    // will ignore any individual errors
    pub async fn insert_many_entries(&self, entries: Vec<ActiveEntry>, master_key: Key<Aes256Gcm>) -> JrnlResult<()> {
        let encrypted_entries = spawn_blocking(move || -> Vec<EncryptedEntry> {
            entries
                .into_iter()
                .filter_map(|entry| ActiveEntry::encrypt(&entry, &master_key).ok())
                .collect::<Vec<_>>()
        })
            .await
            .map_err(Into::<anyhow::Error>::into)?;

        let mut transaction = self.0.begin().await?;

        for entry in encrypted_entries {
            let _ = Self::create_encrypted_entry_query(&entry)
                .execute(&mut *transaction)
                .await;
        }

        transaction.commit().await.map_err(Into::into)
    }
}

pub async fn encrypt_old_entries(pool: PgPool, master_key: Key<Aes256Gcm>) -> anyhow::Result<()> {
    let mut ticker = interval(Duration::from_secs(60 * 5));

    loop {
        ticker.tick().await;

        let mut transaction = pool.begin().await?;

        let entries = sqlx::query_as::<_, ActiveEntry>(
            // language=postgresql
            "DELETE FROM active_entries WHERE expiry < timezone('utc', now()) RETURNING *",
        )
            .fetch_all(&mut *transaction)
            .await?;

        if entries.is_empty() {
            continue;
        }

        let encrypted_entries = match spawn_blocking(move || -> anyhow::Result<_> {
            entries
                .into_iter()
                .map(|entry| ActiveEntry::encrypt(&entry, &master_key))
                .collect::<anyhow::Result<Vec<_>>>()
        })
            .await? {
            Ok(entries) => entries,
            Err(why) => {
                transaction.rollback().await?;
                panic!("failed to encrypt entries in daily task {why:?}");
            }
        };

        for entry in encrypted_entries {
            let Err(why) = EntryService::create_encrypted_entry_query(&entry)
                .execute(&mut *transaction)
                .await else { continue; };

            transaction.rollback().await?;
            panic!("failed to insert encrypted entry in daily task {why:?}");
        }

        transaction.commit().await?;
    }
}