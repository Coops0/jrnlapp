use serde::{Deserialize, Deserializer};

pub mod cursor;

#[allow(clippy::unnecessary_wraps)]
pub fn deserialize_empty_string<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<String>, D::Error> {
    match String::deserialize(deserializer) {
        Ok(str) if str.trim().is_empty() => Ok(None),
        Ok(str) => Ok(Some(str)),
        Err(_) => Ok(None)
    }
}