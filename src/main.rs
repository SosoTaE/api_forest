mod config;
mod routes;
mod state;
mod auth;
mod error;


use crate::state::AppState;
use crate::auth::jwt::JwtKeys;

use anyhow::Context;
use config::Config;
use dotenvy;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tracing_subscriber::EnvFilter;

fn init_tracing(default_filter: &str) {
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_filter));

    tracing_subscriber::fmt().with_env_filter(filter).init();
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Ctrl+C სიგნალის დაყენება ვერ მოხერხდა");
    tracing::info!("მივიღე გაჩერების სიგნალი, ვასრულებ მიმდინარე მოთხოვნებს...");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let config = Config::load("config.toml")?;
    init_tracing(&config.log_level);
    let jwt = JwtKeys::new(&config.jwt_secret, config.access_token_ttl_minutes);


    let pool = PgPoolOptions::new().max_connections(50).connect(&config.database_url).await?;

    let addr = config.addr();
    let state = AppState::new(config, pool, jwt);
    let app = routes::router(state);

    let listener = TcpListener::bind(&addr)
        .await
        .with_context(|| format!("ვერ დავუკავშირდი {addr}-ს"))?;

    tracing::info!(addr = %listener.local_addr()?, "სერვერი ეშვება");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    tracing::info!("სერვერი გაჩერდა");
    Ok(())
}
