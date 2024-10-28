use chrono::{NaiveDate, Utc};
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Profile {
    pub id: Uuid,
    pub theme: String,
    pub first_name: String,
    pub last_name: String,
    pub timezone: String,
}

impl Profile {
    pub fn current_date_by_timezone(&self) -> NaiveDate {
        let tz = self.timezone.parse().unwrap_or(chrono_tz::UTC);
        Utc::now().with_timezone(&tz).date_naive()
    }
}
