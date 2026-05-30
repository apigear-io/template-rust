use std::sync::Arc;
use crate::api::same_struct1_interface::SameStruct1InterfaceTrait;
use crate::implementation::same_struct1_interface::SameStruct1Interface;

/// Shared reference to a SameStruct1Interface implementation.
pub type SharedSameStruct1Interface = Arc<dyn SameStruct1InterfaceTrait>;

/// Creates a new shared SameStruct1Interface with the default implementation.
pub fn new_shared_same_struct1_interface() -> SharedSameStruct1Interface {
    Arc::new(SameStruct1Interface::default())
}
