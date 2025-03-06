use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

use crate::ServiceState;

pub mod db;
pub mod error;
pub mod network;
mod types;

#[derive(OpenApi)]
#[openapi(info(version = "0.2.0"))]
pub struct ApiDoc;

pub fn openapi_router() -> OpenApiRouter<ServiceState> {
    OpenApiRouter::new().nest("/network", network::openapi_router())
}
