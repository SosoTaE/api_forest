use std::sync::Arc;

use sqlx::PgPool;

use crate::{auth::jwt::JwtKeys, config::Config};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub db: PgPool,
    pub started_at: std::time::Instant,   
    pub jwt: Arc<JwtKeys>
}


impl AppState {
    pub fn new(config: Config, db: PgPool, jwt: JwtKeys) -> Self {
        Self {
            config: Arc::new(config),
            db: db,
            jwt: Arc::new(jwt),
            started_at: std::time::Instant::now(),
        }
    }
}