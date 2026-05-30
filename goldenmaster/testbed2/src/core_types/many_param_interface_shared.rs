use std::sync::Arc;
use crate::api::many_param_interface::ManyParamInterfaceTrait;
use crate::implementation::many_param_interface::ManyParamInterface;

/// Shared reference to a ManyParamInterface implementation.
pub type SharedManyParamInterface = Arc<dyn ManyParamInterfaceTrait>;

/// Creates a new shared ManyParamInterface with the default implementation.
pub fn new_shared_many_param_interface() -> SharedManyParamInterface {
    Arc::new(ManyParamInterface::default())
}
