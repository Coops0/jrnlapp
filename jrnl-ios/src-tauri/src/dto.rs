use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Entry {
    pub date: NaiveDate,
    pub emotion_scale: f32,
    pub text: Option<String>,
    #[serde(default)]
    pub saved: bool,
    pub id: String,
}
