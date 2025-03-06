use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    /// Json serialization error.
    #[error("json serde error: {}", source)]
    JsonError {
        /// The source of the error.
        #[from]
        source: serde_json::Error,
    },
}
