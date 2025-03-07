use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::ServiceState;
use crate::api::{db::Neutron, error::ApiError, types::Network};

pub(super) fn openapi_router() -> OpenApiRouter<ServiceState> {
    OpenApiRouter::new().routes(routes!(show))
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
    state
        .db_worker
        .get_network(&state.db, network_id.clone())
        .await
        .map(|x| x.ok_or_else(|| ApiError::NotFound(network_id.clone())))?
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt; // for `collect`
    use sea_orm::DatabaseConnection;
    use std::sync::Arc;
    use tower::ServiceExt; // for `call`, `oneshot`, and `ready`
    use tower_http::trace::TraceLayer;

    use super::*;
    use crate::Service;
    use crate::api::db::MockDbWorker;
    use crate::config::Config;

    #[tokio::test]
    async fn test_show() {
        let mut mock = MockDbWorker::default();
        mock.expect_get_network()
            .returning(|_, _| Ok(Some(Network::default())));

        let db = DatabaseConnection::Disconnected;
        let config = Config::default();
        let service = Service {
            config,
            db,
            db_worker: mock,
        };
        let state = Arc::new(service);

        let mut api = openapi_router()
            .layer(TraceLayer::new_for_http())
            .with_state(state.clone());

        let response = api
            .as_service()
            .oneshot(Request::builder().uri("/id").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let _res: Network = serde_json::from_slice(&body).unwrap();
    }
}
