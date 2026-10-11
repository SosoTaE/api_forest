pub mod models;
pub mod handlers;

use axum::{routing::get, Router};
use crate::state::AppState;


pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/specs",
            get(handlers::list_specs).post(handlers::create_spec),
        )
        .route(
            "/specs/{id}",
            get(handlers::get_spec)
                .patch(handlers::update_spec)
                .delete(handlers::delete_spec),
        )
}