use std::sync::Arc;
use crate::api::enum_interface::EnumInterfaceTrait;
use crate::implementation::enum_interface::EnumInterface;

/// Shared reference to a EnumInterface implementation.
pub type SharedEnumInterface = Arc<dyn EnumInterfaceTrait>;

/// Creates a new shared EnumInterface with the default implementation.
pub fn new_shared_enum_interface() -> SharedEnumInterface {
    Arc::new(EnumInterface::default())
}
