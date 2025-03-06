use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, ToSchema)]
pub struct Network {
    pub id: String,
    pub name: Option<String>,
    pub project_id: String,
    pub tenant_id: String,
    pub status: Option<String>,
    pub shared: bool,
}

impl IntoResponse for Network {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
