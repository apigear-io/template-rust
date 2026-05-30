use std::sync::Arc;
use crate::api::nested_struct2_interface::NestedStruct2InterfaceTrait;
use crate::implementation::nested_struct2_interface::NestedStruct2Interface;

/// Shared reference to a NestedStruct2Interface implementation.
pub type SharedNestedStruct2Interface = Arc<dyn NestedStruct2InterfaceTrait>;

/// Creates a new shared NestedStruct2Interface with the default implementation.
pub fn new_shared_nested_struct2_interface() -> SharedNestedStruct2Interface {
    Arc::new(NestedStruct2Interface::default())
}
