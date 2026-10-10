use anyhow::Context;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: i64,
    pub iat: i64,
}


pub struct JwtKeys {
    encoding: EncodingKey,
    decoding: DecodingKey,
    validation: Validation,
    ttl: Duration,
}

impl JwtKeys {
    pub fn new(secret: &str, ttl_minutes: i64) -> Self {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.leeway = 5;

        Self {
            encoding: EncodingKey::from_secret(secret.as_bytes()),
            decoding: DecodingKey::from_secret(secret.as_bytes()),
            validation,
            ttl: Duration::minutes(ttl_minutes),
        }
    }

    pub fn issue(&self, user_id: Uuid) -> anyhow::Result<String> {
        let now = Utc::now();
        let claims = Claims {
            sub: user_id,
            iat: now.timestamp(),
            exp: (now + self.ttl).timestamp(),
        };

        encode(&Header::new(Algorithm::HS256), &claims, &self.encoding)
            .context("JWT-ის შექმნა ვერ მოხერხდა")
    }


    pub fn verify(&self, token: &str) -> anyhow::Result<Claims> {
        let data = decode::<Claims>(token, &self.decoding, &self.validation)
            .context("JWT არავალიდურია")?;
        Ok(data.claims)
    }

   
    pub fn ttl_seconds(&self) -> i64 {
        self.ttl.num_seconds()
    }
}