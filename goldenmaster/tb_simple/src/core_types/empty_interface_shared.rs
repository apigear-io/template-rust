use std::sync::Arc;
use crate::api::empty_interface::EmptyInterfaceTrait;
use crate::implementation::empty_interface::EmptyInterface;

/// Shared reference to a EmptyInterface implementation.
pub type SharedEmptyInterface = Arc<dyn EmptyInterfaceTrait>;

/// Creates a new shared EmptyInterface with the default implementation.
pub fn new_shared_empty_interface() -> SharedEmptyInterface {
    Arc::new(EmptyInterface::default())
}
