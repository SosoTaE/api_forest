use argon2::{
    password_hash::{PasswordHasher, PasswordVerifier},
    Argon2,
};

pub async fn hash_password(password: String) -> anyhow::Result<String> {
    tokio::task::spawn_blocking(move || -> anyhow::Result<String> {
        let hash = Argon2::default()
            .hash_password(password.as_bytes())
            .map_err(|e| anyhow::anyhow!("ჰეშირება ვერ მოხერხდა: {e}"))?;
        Ok(hash.to_string())
    })
    .await?
}

pub async fn verify_password(password: String, hash: String) -> anyhow::Result<bool> {
    tokio::task::spawn_blocking(move || -> anyhow::Result<bool> {
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), hash.as_str())
            .is_ok())
    })
    .await?
}