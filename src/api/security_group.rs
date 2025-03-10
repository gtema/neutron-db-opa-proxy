use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::ServiceState;
use crate::api::{db::Neutron, error::ApiError, types::SecurityGroup};

pub(super) fn openapi_router() -> OpenApiRouter<ServiceState> {
    OpenApiRouter::new().routes(routes!(show))
}

/// Show security group properties
#[utoipa::path(
    get,
    path = "/{security_group_id}",
    description = "Show security group",
    responses(
        (status = OK, description = "Security group", body = SecurityGroup),
    ),
    tag="security_groups"
)]
#[axum::debug_handler]
#[tracing::instrument(name = "api::get_security_group", level = "debug", skip(state))]
async fn show(
    Path(security_group_id): Path<String>,
    State(state): State<ServiceState>,
) -> Result<impl IntoResponse, ApiError> {
    state
        .db_worker
        .get_security_group(&state.db, &security_group_id)
        .await
        .map(|x| x.ok_or_else(|| ApiError::NotFound(security_group_id.clone())))?
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
        mock.expect_get_security_group()
            .returning(|_, _| Ok(Some(SecurityGroup::default())));

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
        let _res: SecurityGroup = serde_json::from_slice(&body).unwrap();
    }
}
