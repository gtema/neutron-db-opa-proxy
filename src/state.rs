use axum::extract::FromRef;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

use crate::Config;
use crate::ServiceError;

// Placing ServiceState behind Arc is necessary to address DatabaseConnection not implementing
// Clone
#[derive(FromRef)]
pub struct Service {
    pub config: Config,
    #[from_ref(skip)]
    pub db: DatabaseConnection,
}

pub type ServiceState = Arc<Service>;

impl Service {
    pub fn new(cfg: Config, db: DatabaseConnection) -> Result<Self, ServiceError> {
        Ok(Self {
            config: cfg.clone(),
            db,
        })
    }
}
