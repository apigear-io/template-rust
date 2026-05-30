use std::sync::Arc;
use crate::api::same_struct2_interface::SameStruct2InterfaceTrait;
use crate::implementation::same_struct2_interface::SameStruct2Interface;

/// Shared reference to a SameStruct2Interface implementation.
pub type SharedSameStruct2Interface = Arc<dyn SameStruct2InterfaceTrait>;

/// Creates a new shared SameStruct2Interface with the default implementation.
pub fn new_shared_same_struct2_interface() -> SharedSameStruct2Interface {
    Arc::new(SameStruct2Interface::default())
}
