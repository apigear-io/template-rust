use std::sync::Arc;
use crate::api::nested_struct3_interface::NestedStruct3InterfaceTrait;
use crate::implementation::nested_struct3_interface::NestedStruct3Interface;

/// Shared reference to a NestedStruct3Interface implementation.
pub type SharedNestedStruct3Interface = Arc<dyn NestedStruct3InterfaceTrait>;

/// Creates a new shared NestedStruct3Interface with the default implementation.
pub fn new_shared_nested_struct3_interface() -> SharedNestedStruct3Interface {
    Arc::new(NestedStruct3Interface::default())
}
