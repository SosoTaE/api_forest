use crate::state::AppState;
use serde::Serialize;
use axum::{extract::State, response::IntoResponse, Json};


#[derive(Serialize )]
pub struct HealthResponse {
    status: String,
    uptime_seconds: u64,
    version: String,
}

pub async fn health_check(State(state): State<AppState>) -> impl IntoResponse {
    let uptime_seconds = state.started_at.elapsed().as_secs();

    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds,
    })
}


