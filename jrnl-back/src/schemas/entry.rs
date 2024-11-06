use chrono::NaiveDate;
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Entry {
    pub id: Uuid,
    pub author: Uuid,
    pub date: NaiveDate,
    pub emotion_scale: f32,
    pub text: Option<String>,
}
