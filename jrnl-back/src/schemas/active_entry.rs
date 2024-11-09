use crate::schemas::entry::EncryptedEntry;
use aes_gcm::aead::{Aead, OsRng};
use aes_gcm::{AeadCore, Aes256Gcm, Key, KeyInit};
use anyhow::anyhow;
use chrono::NaiveDate;
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct ActiveEntry {
    pub id: Uuid,
    pub author: Uuid,
    pub date: NaiveDate,
    pub emotion_scale: f32,
    pub text: Option<String>,
}

impl ActiveEntry {
    pub fn encrypt(mut self, master_cipher: Aes256Gcm) -> anyhow::Result<EncryptedEntry> {
        let key = Aes256Gcm::generate_key(OsRng);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

        let encrypted_content_key = master_cipher.encrypt(&nonce, &key[..]).map_err(|e| anyhow!(e))?;
        let content_key_cipher = Aes256Gcm::new(
            Key::<Aes256Gcm>::from_slice(&encrypted_content_key)
        );

        let encrypted_content = content_key_cipher.encrypt(
            &nonce,
            self.text.take().unwrap_or_default().as_bytes(),
        ).map_err(|e| anyhow!(e))?;
        

        Ok(EncryptedEntry {
            id: self.id,
            author: self.author,
            date: self.date,
            emotion_scale: self.emotion_scale,
            encrypted_content,
            content_key: encrypted_content_key,
            nonce: nonce.to_vec(),
        })
    }
}