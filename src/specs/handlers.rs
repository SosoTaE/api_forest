use super::models::{CreateSpec, ListParams, Spec, UpdateSpec};
use crate::{
    auth::CurrentUser,
    error::{AppError, AppResult},
    state::AppState,
};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::{StatusCode, header},
    response::IntoResponse,
};
use uuid::Uuid;
use validator::Validate;

pub async fn create_spec(
    State(state): State<AppState>,
    user: CurrentUser,
    Json(input): Json<CreateSpec>,
) -> AppResult<impl IntoResponse> {
    input
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let spec = sqlx::query_as!(
        Spec,
        r#"
        INSERT INTO specs (name, description, owner_id)
        VALUES ($1, $2, $3)
        RETURNING id, name, description, created_at, updated_at
        "#,
        input.name,
        input.description,
        user.id
    )
    .fetch_one(&state.db)
    .await?;

    let location = format!("/specs/{}", spec.id);

    Ok((
        StatusCode::CREATED,
        [(header::LOCATION, location)],
        Json(spec),
    ))
}

pub async fn list_specs(
    State(state): State<AppState>,
    user: CurrentUser,
    Query(params): Query<ListParams>,
) -> AppResult<impl IntoResponse> {
    let limit = params.limit.unwrap_or(20).clamp(1, 100);

    let specs = sqlx::query_as!(
        Spec,
        r#"SELECT id, name, description, created_at, updated_at
        FROM specs
        WHERE owner_id = $1
        ORDER BY created_at DESC
        LIMIT $2"#,
        user.id,
        limit,
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(specs))
}

pub async fn get_spec(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    let spec = sqlx::query_as!(
        Spec,
        r#"
        SELECT id, name, description, created_at, updated_at
        FROM specs
        WHERE id = $1 AND owner_id = $2
        "#,
        id,
        user.id,
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(spec))
}

pub async fn update_spec(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<Uuid>,
    Json(update_spec_data): Json<UpdateSpec>,
) -> AppResult<impl IntoResponse> {
    update_spec_data
        .validate()
        .map_err(|e| AppError::Validation(e.to_string()))?;

    let spec = sqlx::query_as!(
        Spec,
        r#"
        UPDATE specs
        SET name = COALESCE($3, name),
        description = COALESCE($4, description),
        updated_at  = now()
        WHERE id = $1 AND owner_id = $2
        RETURNING id, name, description, created_at, updated_at
        "#,
        id,
        user.id,
        update_spec_data.name,
        update_spec_data.description
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(spec))
}


pub async fn delete_spec(
    State(state): State<AppState>,
    user: CurrentUser,
    Path(id): Path<Uuid>,
) -> AppResult<impl IntoResponse> {
    let spec = sqlx::query!(
        r#"
        DELETE FROM specs
        WHERE id = $1 AND owner_id = $2
        "#,
        id,
        user.id,
    )
    .execute(&state.db)
    .await?;

    if spec.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(StatusCode::NO_CONTENT)
}
