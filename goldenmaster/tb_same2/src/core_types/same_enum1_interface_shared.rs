use std::sync::Arc;
use crate::api::same_enum1_interface::SameEnum1InterfaceTrait;
use crate::implementation::same_enum1_interface::SameEnum1Interface;

/// Shared reference to a SameEnum1Interface implementation.
pub type SharedSameEnum1Interface = Arc<dyn SameEnum1InterfaceTrait>;

/// Creates a new shared SameEnum1Interface with the default implementation.
pub fn new_shared_same_enum1_interface() -> SharedSameEnum1Interface {
    Arc::new(SameEnum1Interface::default())
}
