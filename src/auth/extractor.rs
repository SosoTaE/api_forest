use axum::{
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts},
};
use uuid::Uuid;

use crate::{auth::jwt::Claims, error::AppError, state::AppState};


pub struct CurrentUser {
    pub id: Uuid,
}

impl FromRequestParts<AppState> for CurrentUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.strip_prefix("Bearer "))
            .ok_or(AppError::Unauthorized)?;

        let claims: Claims = state
            .jwt
            .verify(token)
            .map_err(|_| AppError::Unauthorized)?;

 
        

        Ok(CurrentUser { id: claims.sub})
    }
}