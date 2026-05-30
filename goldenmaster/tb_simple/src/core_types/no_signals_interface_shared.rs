use std::sync::Arc;
use crate::api::no_signals_interface::NoSignalsInterfaceTrait;
use crate::implementation::no_signals_interface::NoSignalsInterface;

/// Shared reference to a NoSignalsInterface implementation.
pub type SharedNoSignalsInterface = Arc<dyn NoSignalsInterfaceTrait>;

/// Creates a new shared NoSignalsInterface with the default implementation.
pub fn new_shared_no_signals_interface() -> SharedNoSignalsInterface {
    Arc::new(NoSignalsInterface::default())
}
