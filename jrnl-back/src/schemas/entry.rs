use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce};
use anyhow::anyhow;
use chrono::NaiveDate;
use serde::Serialize;
use sqlx::FromRow;
use tokio::task::spawn_blocking;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct EncryptedEntry {
    pub id: Uuid,
    pub author: Uuid,
    pub date: NaiveDate,
    pub emotion_scale: f32,
    pub encrypted_content: Vec<u8>,
    pub content_key: Vec<u8>,
    pub nonce: Vec<u8>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DecryptedEntry {
    pub id: Uuid,
    pub author: Uuid,
    pub date: NaiveDate,
    pub emotion_scale: f32,
    pub text: Option<String>,
}

impl EncryptedEntry {
    pub async fn decrypt(self, master_cipher: Aes256Gcm) -> anyhow::Result<DecryptedEntry> {
        let decrypted_content_bytes = spawn_blocking(move || -> anyhow::Result<Vec<u8>> {
            let nonce = Nonce::from_slice(&self.nonce[..]);
            let decrypted_content_key = master_cipher.decrypt(nonce, &self.content_key[..]).map_err(|e| anyhow!(e))?;

            let content_key_cipher = Aes256Gcm::new(
                Key::<Aes256Gcm>::from_slice(&decrypted_content_key)
            );

            content_key_cipher.decrypt(nonce, &self.encrypted_content[..]).map_err(|e| anyhow!(e))
        }).await??;

        let decrypted_content = String::from_utf8(decrypted_content_bytes)?;
        let text = if decrypted_content.is_empty() {
            None
        } else {
            Some(decrypted_content)
        };

        Ok(DecryptedEntry {
            id: self.id,
            author: self.author,
            date: self.date,
            emotion_scale: self.emotion_scale,
            text,
        })
    }
}