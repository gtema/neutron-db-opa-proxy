use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Network resource
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, ToSchema)]
pub struct Network {
    /// Network ID
    pub id: String,
    /// Network name
    pub name: Option<String>,
    /// Project ID of a network
    pub project_id: String,
    /// Project ID of a network
    pub tenant_id: String,
    /// Network status
    pub status: Option<String>,
    /// Flag indicating whether the network is shared or not
    pub shared: bool,
}

impl IntoResponse for Network {
    fn into_response(self) -> Response {
        (
            StatusCode::OK,
            [("Cache-Control", "max-age=604800")],
            Json(self),
        )
            .into_response()
    }
}

/// Subnet resource
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, ToSchema)]
pub struct Subnet {
    /// Subnet ID
    pub id: String,
    /// Subnet name
    pub name: Option<String>,
    /// Network id
    pub network_id: String,
    /// Project ID of a subnet
    pub project_id: Option<String>,
    /// Project ID of a subnet
    pub tenant_id: Option<String>,
}

impl IntoResponse for Subnet {
    fn into_response(self) -> Response {
        (
            StatusCode::OK,
            [("Cache-Control", "max-age=604800")],
            Json(self),
        )
            .into_response()
    }
}

/// Security group resource
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, ToSchema)]
pub struct SecurityGroup {
    /// Seurity group ID
    pub id: String,
    /// Security group name
    pub name: Option<String>,
    /// Project ID of a security group
    pub project_id: Option<String>,
    /// Project ID of a security group
    pub tenant_id: Option<String>,
}

impl IntoResponse for SecurityGroup {
    fn into_response(self) -> Response {
        (
            StatusCode::OK,
            [("Cache-Control", "max-age=604800")],
            Json(self),
        )
            .into_response()
    }
}

/// Floating IPresource
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, ToSchema)]
pub struct FloatingIP {
    /// Floating IP ID
    pub id: String,
    /// Floating IP address
    pub floating_ip_address: String,
    /// Project ID of a floating ip
    pub project_id: Option<String>,
    /// Project ID of a floating ip
    pub tenant_id: Option<String>,
    /// Status
    pub status: Option<String>,
}

impl IntoResponse for FloatingIP {
    fn into_response(self) -> Response {
        (
            StatusCode::OK,
            [("Cache-Control", "max-age=604800")],
            Json(self),
        )
            .into_response()
    }
}
