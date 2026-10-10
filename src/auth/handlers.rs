use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::{
    auth::{password, refresh, CurrentUser},
    error::{AppError, AppResult},
    state::AppState,
};

#[derive(Deserialize, Validate)]
pub struct Credentials {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 128))]
    pub password: String,
}


#[derive(sqlx::FromRow)]
struct UserRow {
    id: Uuid,
    email: String,
    password_hash: String,
    display_name: Option<String>,
    created_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub display_name: Option<String>,
    pub created_at: DateTime<Utc>,
}


impl From<UserRow> for UserResponse {
    fn from(row: UserRow) -> Self {
        Self {
            id: row.id,
            email: row.email,
            display_name: row.display_name,
            created_at: row.created_at,
        }
    }
}

#[derive(Serialize)]
pub struct TokenResponse {
    access_token: String,
    token_type: &'static str,
    expires_in: i64,
    refresh_token: String,
}


pub async fn register(
    State(state): State<AppState>,
    Json(input): Json<Credentials>,
) -> AppResult<impl IntoResponse> {
    input.validate().map_err(|e| AppError::Validation(e.to_string()))?;

    let email = input.email.trim().to_lowercase();
    let password_hash = password::hash_password(input.password).await?;

    let user = sqlx::query_as!(
        UserRow,
        r#"
        INSERT INTO users (email, password_hash)
        VALUES ($1, $2)
        RETURNING id, email, password_hash, display_name, created_at
        "#,
        email,
        password_hash,
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(db) if db.is_unique_violation() => {
            AppError::Conflict("ეს email უკვე დაკავებულია".into())
        }
        other => other.into(),
    })?;

    Ok((StatusCode::CREATED, Json(UserResponse::from(user))))
}


pub async fn login(
    State(state): State<AppState>,
    Json(input): Json<Credentials>,
) -> AppResult<Json<TokenResponse>> {
    let email = input.email.trim().to_lowercase();

    let user = sqlx::query_as!(
        UserRow,
        r#"
        SELECT id, email, password_hash, display_name, created_at
        FROM users
        WHERE lower(email) = $1
        "#,
        email,
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or(AppError::Unauthorized)?;

    let valid = password::verify_password(input.password, user.password_hash).await?;
    if !valid {
        return Err(AppError::Unauthorized);
    }

    let tokens = issue_tokens(&state, user.id).await?;
    Ok(Json(tokens))
}

async fn issue_tokens(state: &AppState, user_id: Uuid) -> AppResult<TokenResponse> {
    let refresh_token =
        refresh::store(&state.db, user_id, state.config.refresh_token_ttl_days).await?;

    Ok(TokenResponse {
        access_token: state.jwt.issue(user_id)?,
        token_type: "Bearer",
        expires_in: state.jwt.ttl_seconds(),
        refresh_token,
    })
}


#[derive(Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}


pub async fn refresh(
    State(state): State<AppState>,
    Json(input): Json<RefreshRequest>,
) -> AppResult<Json<TokenResponse>> {
    let mut tx: sqlx::Transaction<'_, sqlx::Postgres> = state.db.begin().await?;


    let row = sqlx::query!(
        r#"
        UPDATE refresh_tokens
        SET revoked_at = now()
        WHERE token_hash = $1
          AND revoked_at IS NULL
          AND expires_at > now()
        RETURNING user_id
        "#,
        refresh::hash(&input.refresh_token),
    )
    .fetch_optional(&mut *tx)
    .await?
    .ok_or(AppError::Unauthorized)?;

    let new_refresh =
        refresh::store(&mut *tx, row.user_id, state.config.refresh_token_ttl_days).await?;

    tx.commit().await?;

    Ok(Json(TokenResponse {
        access_token: state.jwt.issue(row.user_id)?,
        token_type: "Bearer",
        expires_in: state.jwt.ttl_seconds(),
        refresh_token: new_refresh,
    }))
}


pub async fn logout(
    State(state): State<AppState>,
    Json(input): Json<RefreshRequest>,
) -> AppResult<StatusCode> {
    sqlx::query!(
        "UPDATE refresh_tokens SET revoked_at = now() WHERE token_hash = $1 AND revoked_at IS NULL",
        refresh::hash(&input.refresh_token),
    )
    .execute(&state.db)
    .await?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn me(
    State(state): State<AppState>,
    user: CurrentUser,
) -> AppResult<Json<UserResponse>> {
    let row = sqlx::query_as!(
        UserRow,
        r#"
        SELECT id, email, password_hash, display_name, created_at
        FROM users
        WHERE id = $1
        "#,
        user.id,
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(row.into()))
}