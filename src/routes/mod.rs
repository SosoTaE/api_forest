mod health;

use axum::{
    Router,
    http::StatusCode,
    routing::{get, post},
};
use std::time::Duration;
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};

use crate::auth;
use crate::state::AppState;

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health::health_check))
        .route("/auth/register", post(auth::handlers::register))
        .route("/auth/login", post(auth::handlers::login))
        .route("/auth/refresh", post(auth::handlers::refresh))
        .route("/auth/logout", post(auth::handlers::logout))
        .route("/users/me", get(auth::handlers::me))
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(10),
        ))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
