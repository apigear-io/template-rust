use std::sync::Arc;
use crate::api::struct_array_interface::StructArrayInterfaceTrait;
use crate::implementation::struct_array_interface::StructArrayInterface;

/// Shared reference to a StructArrayInterface implementation.
pub type SharedStructArrayInterface = Arc<dyn StructArrayInterfaceTrait>;

/// Creates a new shared StructArrayInterface with the default implementation.
pub fn new_shared_struct_array_interface() -> SharedStructArrayInterface {
    Arc::new(StructArrayInterface::default())
}
