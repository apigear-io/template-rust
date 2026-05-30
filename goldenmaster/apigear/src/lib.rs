use std::future::Future;
use std::pin::Pin;

pub use parking_lot;
pub use tokio;
pub use tracing;

/// A boxed, pinned future returned by async trait operations.
/// Named `ApiFuture` (not `ApiResult`) because it wraps a `Future`, not a `Result`.
pub type ApiFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Error type for API operations.
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("operation failed: {0}")]
    OperationFailed(String),
    #[error("not connected")]
    NotConnected,
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("transport error: {0}")]
    Transport(String),
}
