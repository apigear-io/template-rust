use std::sync::Arc;
use crate::api::void_interface::VoidInterfaceTrait;
use crate::implementation::void_interface::VoidInterface;

/// Shared reference to a VoidInterface implementation.
pub type SharedVoidInterface = Arc<dyn VoidInterfaceTrait>;

/// Creates a new shared VoidInterface with the default implementation.
pub fn new_shared_void_interface() -> SharedVoidInterface {
    Arc::new(VoidInterface::default())
}
