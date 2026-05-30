use std::sync::Arc;
use crate::api::nam_es::NamEsTrait;
use crate::implementation::nam_es::NamEs;

/// Shared reference to a NamEs implementation.
pub type SharedNamEs = Arc<dyn NamEsTrait>;

/// Creates a new shared NamEs with the default implementation.
pub fn new_shared_nam_es() -> SharedNamEs {
    Arc::new(NamEs::default())
}
