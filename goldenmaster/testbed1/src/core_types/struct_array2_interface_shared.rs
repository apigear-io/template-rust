use std::sync::Arc;
use crate::api::struct_array2_interface::StructArray2InterfaceTrait;
use crate::implementation::struct_array2_interface::StructArray2Interface;

/// Shared reference to a StructArray2Interface implementation.
pub type SharedStructArray2Interface = Arc<dyn StructArray2InterfaceTrait>;

/// Creates a new shared StructArray2Interface with the default implementation.
pub fn new_shared_struct_array2_interface() -> SharedStructArray2Interface {
    Arc::new(StructArray2Interface::default())
}
