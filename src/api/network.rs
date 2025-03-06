use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::ServiceState;
use crate::api::error::ApiError;

pub(super) fn openapi_router() -> OpenApiRouter<ServiceState> {
    OpenApiRouter::new().routes(routes!(show))
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, ToSchema)]
pub struct Network {
    /// Network ID
    pub id: String,
}

impl IntoResponse for Network {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

/// Show network properties
#[utoipa::path(
    get,
    path = "/{network_id}",
    description = "Show network",
    responses(
        (status = OK, description = "Network", body = Network),
    ),
    tag="networks"
)]
#[axum::debug_handler]
#[tracing::instrument(name = "api::get_network", level = "debug", skip(state))]
async fn show(
    Path(network_id): Path<String>,
    State(state): State<ServiceState>,
) -> Result<impl IntoResponse, ApiError> {
    //let assignments: Result<Vec<Assignment>, _> = state
    //    .provider
    //    .get_assignment_provider()
    //    .list_role_assignments(&state.db, &query.try_into()?)
    //    .await
    //    .map_err(KeystoneApiError::assignment)?
    //    .into_iter()
    //    .map(TryInto::try_into)
    //    .collect();
    Ok(Network {
        id: network_id.clone(),
    })
}
