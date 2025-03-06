use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

/// API operation errors
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("database error: {}", source)]
    Database {
        #[from]
        source: sea_orm::DbErr,
    },

    #[error("resource with id {0} not found")]
    NotFound(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            Self::NotFound(_) => (
                StatusCode::NOT_FOUND,
                Json(json!({"error": {"code": StatusCode::NOT_FOUND.as_u16(), "message": self.to_string()}})),
            )
                .into_response(),
            Self::Database{..} => (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": {"code": StatusCode::BAD_REQUEST.as_u16(), "message": self.to_string()}})),
            )
                .into_response(),
        }
    }
}
