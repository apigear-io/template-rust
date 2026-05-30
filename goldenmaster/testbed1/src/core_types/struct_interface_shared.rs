use std::sync::Arc;
use crate::api::struct_interface::StructInterfaceTrait;
use crate::implementation::struct_interface::StructInterface;

/// Shared reference to a StructInterface implementation.
pub type SharedStructInterface = Arc<dyn StructInterfaceTrait>;

/// Creates a new shared StructInterface with the default implementation.
pub fn new_shared_struct_interface() -> SharedStructInterface {
    Arc::new(StructInterface::default())
}
