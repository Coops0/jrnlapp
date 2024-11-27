use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Entry {
    #[serde(deserialize_with = "deserialize_naive_date")]
    pub date: NaiveDate,
    pub emotion_scale: f32,
    pub text: Option<String>,
    #[serde(default)]
    pub saved: bool,
    pub id: String,
    #[serde(default)]
    pub ephemeral: bool,
}

fn deserialize_naive_date<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<NaiveDate, D::Error> {
    let s = String::deserialize(deserializer)?;
    NaiveDate::parse_from_str(&s, "%m/%d/%Y").map_err(serde::de::Error::custom)
}
