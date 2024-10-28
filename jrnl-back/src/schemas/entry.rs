use serde::Serialize;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Entry {
    pub id: uuid::Uuid,
    pub author: uuid::Uuid,
    pub date: chrono::NaiveDate,
    pub emotion_scale: f32,
    pub text: Option<String>
}