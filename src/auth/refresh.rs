use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use chrono::{Duration, Utc};
use rand::RngExt;
use sha2::{Digest, Sha256};
use uuid::Uuid;

pub fn generate() -> String {
    let bytes: [u8; 32] = rand::rng().random();
    URL_SAFE_NO_PAD.encode(bytes)
}

pub fn hash(token: &str) -> String {
    URL_SAFE_NO_PAD.encode(Sha256::digest(token.as_bytes()))
}


pub async fn store(
    executor: impl sqlx::PgExecutor<'_>,
    user_id: Uuid,
    ttl_days: i64,
) -> Result<String, sqlx::Error> {
    let token = generate();
    let expires_at = Utc::now() + Duration::days(ttl_days);

    sqlx::query!(
        "INSERT INTO refresh_tokens (user_id, token_hash, expires_at) VALUES ($1, $2, $3)",
        user_id,
        hash(&token),
        expires_at,
    )
    .execute(executor)
    .await?;

    Ok(token)
}