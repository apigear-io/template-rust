use std::sync::Arc;
use crate::api::simple_array_interface::SimpleArrayInterfaceTrait;
use crate::implementation::simple_array_interface::SimpleArrayInterface;

/// Shared reference to a SimpleArrayInterface implementation.
pub type SharedSimpleArrayInterface = Arc<dyn SimpleArrayInterfaceTrait>;

/// Creates a new shared SimpleArrayInterface with the default implementation.
pub fn new_shared_simple_array_interface() -> SharedSimpleArrayInterface {
    Arc::new(SimpleArrayInterface::default())
}
