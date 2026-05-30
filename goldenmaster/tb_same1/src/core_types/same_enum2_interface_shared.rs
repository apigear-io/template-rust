use std::sync::Arc;
use crate::api::same_enum2_interface::SameEnum2InterfaceTrait;
use crate::implementation::same_enum2_interface::SameEnum2Interface;

/// Shared reference to a SameEnum2Interface implementation.
pub type SharedSameEnum2Interface = Arc<dyn SameEnum2InterfaceTrait>;

/// Creates a new shared SameEnum2Interface with the default implementation.
pub fn new_shared_same_enum2_interface() -> SharedSameEnum2Interface {
    Arc::new(SameEnum2Interface::default())
}
