use apigear::{ApiError, ApiFuture};
use crate::api::no_signals_interface::NoSignalsInterfacePublisher;
use crate::api::no_signals_interface::NoSignalsInterfaceTrait;
use tracing;

/// Trace decorator for NoSignalsInterface.
/// Wraps any implementation and instruments all operations with tracing spans.
pub struct NoSignalsInterfaceTraced<T: NoSignalsInterfaceTrait> {
    inner: T,
}

impl<T: NoSignalsInterfaceTrait> NoSignalsInterfaceTraced<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: NoSignalsInterfaceTrait> NoSignalsInterfaceTrait for NoSignalsInterfaceTraced<T> {
    fn func_void(&self) -> ApiFuture<'_, Result<(), ApiError>> {
        tracing::info!("NoSignalsInterface::func_void called");
        self.inner.func_void()
    }

    fn func_bool(
        &self,
        param_bool: bool,
    ) -> ApiFuture<'_, Result<bool, ApiError>> {
        tracing::info!("NoSignalsInterface::func_bool called");
        self.inner.func_bool(param_bool)
    }

    fn prop_bool(&self) -> bool {
        self.inner.prop_bool()
    }
    fn set_prop_bool(
        &self,
        prop_bool: bool,
    ) {
        tracing::info!("NoSignalsInterface::set_prop_bool called");
        self.inner.set_prop_bool(prop_bool);
    }

    fn prop_int(&self) -> i32 {
        self.inner.prop_int()
    }
    fn set_prop_int(
        &self,
        prop_int: i32,
    ) {
        tracing::info!("NoSignalsInterface::set_prop_int called");
        self.inner.set_prop_int(prop_int);
    }

    fn publisher(&self) -> &NoSignalsInterfacePublisher {
        self.inner.publisher()
    }
}
