use chrono::{DateTime, NaiveDate, Utc};
use chrono_tz::Tz;
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub google_subject: Option<String>,
    pub apple_subject: Option<String>,
    pub theme: Option<String>,
    pub timezone: String,
    pub has_had_tour: bool,
    pub has_seen_app_push: bool
}

impl User {
    pub fn timezone(&self) -> Tz {
        self.timezone.parse().unwrap_or(chrono_tz::UTC)
    }

    pub fn current_date_time_by_timezone(&self) -> DateTime<Tz> {
        Utc::now().with_timezone(&self.timezone())
    }

    pub fn current_date_by_timezone(&self) -> NaiveDate {
        self.current_date_time_by_timezone().date_naive()
    }
}
