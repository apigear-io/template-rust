use std::sync::Arc;
use crate::api::simple_interface::SimpleInterfaceTrait;
use crate::implementation::simple_interface::SimpleInterface;

/// Shared reference to a SimpleInterface implementation.
pub type SharedSimpleInterface = Arc<dyn SimpleInterfaceTrait>;

/// Creates a new shared SimpleInterface with the default implementation.
pub fn new_shared_simple_interface() -> SharedSimpleInterface {
    Arc::new(SimpleInterface::default())
}
