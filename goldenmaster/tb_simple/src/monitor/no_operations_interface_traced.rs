use crate::api::no_operations_interface::NoOperationsInterfacePublisher;
use crate::api::no_operations_interface::NoOperationsInterfaceTrait;
use tracing;

/// Trace decorator for NoOperationsInterface.
/// Wraps any implementation and instruments all operations with tracing spans.
pub struct NoOperationsInterfaceTraced<T: NoOperationsInterfaceTrait> {
    inner: T,
}

impl<T: NoOperationsInterfaceTrait> NoOperationsInterfaceTraced<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: NoOperationsInterfaceTrait> NoOperationsInterfaceTrait for NoOperationsInterfaceTraced<T> {
    fn prop_bool(&self) -> bool {
        self.inner.prop_bool()
    }
    fn set_prop_bool(
        &self,
        prop_bool: bool,
    ) {
        tracing::info!("NoOperationsInterface::set_prop_bool called");
        self.inner.set_prop_bool(prop_bool);
    }

    fn prop_int(&self) -> i32 {
        self.inner.prop_int()
    }
    fn set_prop_int(
        &self,
        prop_int: i32,
    ) {
        tracing::info!("NoOperationsInterface::set_prop_int called");
        self.inner.set_prop_int(prop_int);
    }

    fn publisher(&self) -> &NoOperationsInterfacePublisher {
        self.inner.publisher()
    }
}
