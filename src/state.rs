use axum::extract::FromRef;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use mockall_double::double;

#[double]
use crate::api::db::DbWorker;
use crate::Config;
use crate::ServiceError;

// Placing ServiceState behind Arc is necessary to address DatabaseConnection not implementing
// Clone
#[derive(FromRef)]
pub struct Service {
    pub(crate) config: Config,
    #[from_ref(skip)]
    pub(crate) db: DatabaseConnection,
    pub(crate) db_worker: DbWorker,
}

pub type ServiceState = Arc<Service>;

impl Service {
    pub fn new(cfg: Config, db: DatabaseConnection) -> Result<Self, ServiceError> {
        Ok(Self {
            config: cfg.clone(),
            db,
            db_worker: DbWorker::default()
        })
    }
}
