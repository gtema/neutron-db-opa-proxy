use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

use crate::ServiceState;

pub mod db;
pub mod error;
pub mod floating_ip;
pub mod network;
pub mod security_group;
mod types;

#[derive(OpenApi)]
#[openapi(info(version = "0.2.0"))]
pub struct ApiDoc;

pub fn openapi_router() -> OpenApiRouter<ServiceState> {
    OpenApiRouter::new()
        .nest("/floating_ip", floating_ip::openapi_router())
        .nest("/network", network::openapi_router())
        .nest("/security_group", security_group::openapi_router())
}
