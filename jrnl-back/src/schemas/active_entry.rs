use crate::schemas::entry::EncryptedEntry;
use aes_gcm::{
    aead::{Aead, OsRng},
    AeadCore,
    Aes256Gcm,
    Key,
    KeyInit,
};
use anyhow::{anyhow, bail};
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
    pub expiry: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub ephemeral: bool,
}

impl ActiveEntry {
    pub fn encrypt(&self, master_key: &Key<Aes256Gcm>) -> anyhow::Result<EncryptedEntry> {
        if self.ephemeral {
            bail!("cannot encrypt ephemeral entry");
        }

        let master_cipher = Aes256Gcm::new(master_key);

        let key = Aes256Gcm::generate_key(OsRng);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

        let content_key_cipher = Aes256Gcm::new(&key);

        let encrypted_content_key = master_cipher
            .encrypt(&nonce, &key[..])
            .map_err(|_| anyhow!("failed to encrypt content key"))?;

        let encrypted_content = content_key_cipher
            .encrypt(&nonce, self.text.as_deref().unwrap_or_default().as_bytes())
            .map_err(|_| anyhow!("failed to encrypt entry content"))?;

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
