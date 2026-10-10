use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("რესურსი ვერ მოიძებნა")]
    NotFound,

    #[error("არასწორი მონაცემები: {0}")]
    Validation(String),

    #[error(transparent)]
    Database(#[from] sqlx::Error),

    #[error(transparent)]
    Internal(#[from] anyhow::Error),
    #[error("ავტორიზაცია საჭიროა")]
    Unauthorized,

    #[error("{0}")]
    Conflict(String),
}

/// RFC 9457: application/problem+json
#[derive(Serialize)]
struct Problem {
    title: &'static str,
    status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<String>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, title, detail) = match &self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not Found", None),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized", None),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, "Conflict", Some(msg.clone())),
            AppError::Validation(msg) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "Validation Failed",
                Some(msg.clone()),
            ),
            AppError::Database(sqlx::Error::RowNotFound) => {
                (StatusCode::NOT_FOUND, "Not Found", None)
            }
            AppError::Database(_) | AppError::Internal(_) => {
                tracing::error!(error = ?self, "შიდა შეცდომა");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error",
                    None,
                )
            }
        };

        let body = Problem {
            title,
            status: status.as_u16(),
            detail,
        };
        let mut response = (status, Json(body)).into_response();
        response.headers_mut().insert(
            axum::http::header::CONTENT_TYPE,
            "application/problem+json".parse().unwrap(),
        );

        if status == StatusCode::UNAUTHORIZED {
        response.headers_mut().insert(
            axum::http::header::WWW_AUTHENTICATE,
            axum::http::HeaderValue::from_static("Bearer"),
        );
}

        response
    }
}

pub type AppResult<T> = Result<T, AppError>;
