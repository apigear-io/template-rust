use std::sync::Arc;
use crate::api::nested_struct1_interface::NestedStruct1InterfaceTrait;
use crate::implementation::nested_struct1_interface::NestedStruct1Interface;

/// Shared reference to a NestedStruct1Interface implementation.
pub type SharedNestedStruct1Interface = Arc<dyn NestedStruct1InterfaceTrait>;

/// Creates a new shared NestedStruct1Interface with the default implementation.
pub fn new_shared_nested_struct1_interface() -> SharedNestedStruct1Interface {
    Arc::new(NestedStruct1Interface::default())
}
