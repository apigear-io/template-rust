use std::sync::Arc;
use crate::api::no_operations_interface::NoOperationsInterfaceTrait;
use crate::implementation::no_operations_interface::NoOperationsInterface;

/// Shared reference to a NoOperationsInterface implementation.
pub type SharedNoOperationsInterface = Arc<dyn NoOperationsInterfaceTrait>;

/// Creates a new shared NoOperationsInterface with the default implementation.
pub fn new_shared_no_operations_interface() -> SharedNoOperationsInterface {
    Arc::new(NoOperationsInterface::default())
}
