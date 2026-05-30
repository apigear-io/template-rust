use std::sync::Arc;
use crate::api::no_properties_interface::NoPropertiesInterfaceTrait;
use crate::implementation::no_properties_interface::NoPropertiesInterface;

/// Shared reference to a NoPropertiesInterface implementation.
pub type SharedNoPropertiesInterface = Arc<dyn NoPropertiesInterfaceTrait>;

/// Creates a new shared NoPropertiesInterface with the default implementation.
pub fn new_shared_no_properties_interface() -> SharedNoPropertiesInterface {
    Arc::new(NoPropertiesInterface::default())
}
