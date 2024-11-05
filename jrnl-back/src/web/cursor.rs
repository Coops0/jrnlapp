use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use chrono::NaiveDate;
use serde::{Deserialize, Deserializer, Serialize};
use uuid::Uuid;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct Cursor {
    pub date: NaiveDate,
    pub id: Uuid,
}

impl Default for Cursor {
    fn default() -> Self {
        Self {
            date: NaiveDate::MAX,
            id: Uuid::max(),
        }
    }
}

impl Cursor {
    fn encode(&self) -> String {
        STANDARD.encode(format!("{}:{}", self.date, self.id))
    }

    fn decode(s: &str) -> Option<Self> {
        STANDARD
            .decode(s)
            .ok()
            .and_then(|b| String::from_utf8(b).ok())
            .and_then(|c| {
                let (date, id) = c.split_once(':')?;
                Some(Self {
                    date: NaiveDate::parse_from_str(date, "%Y-%m-%d").ok()?,
                    id: Uuid::parse_str(id).ok()?,
                })
            })
    }
}

impl<'de> Deserialize<'de> for Cursor {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Self::decode(&s).ok_or_else(|| serde::de::Error::custom("invalid cursor"))
    }
}

impl Serialize for Cursor {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.encode().serialize(serializer)
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Deserialize)]
pub struct CursorParams {
    pub cursor: Option<Cursor>,
    pub limit: Option<u32>,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Serialize)]
pub struct CursorPaginatedResponse<T> {
    pub items: Vec<T>,
    pub next_cursor: Option<Cursor>,
    pub has_more: bool,
}
