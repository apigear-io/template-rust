use apigear::{ApiError, ApiFuture};
use crate::api::no_properties_interface::NoPropertiesInterfacePublisher;
use crate::api::no_properties_interface::NoPropertiesInterfaceTrait;
use tracing;

/// Trace decorator for NoPropertiesInterface.
/// Wraps any implementation and instruments all operations with tracing spans.
pub struct NoPropertiesInterfaceTraced<T: NoPropertiesInterfaceTrait> {
    inner: T,
}

impl<T: NoPropertiesInterfaceTrait> NoPropertiesInterfaceTraced<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: NoPropertiesInterfaceTrait> NoPropertiesInterfaceTrait for NoPropertiesInterfaceTraced<T> {
    fn func_void(&self) -> ApiFuture<'_, Result<(), ApiError>> {
        tracing::info!("NoPropertiesInterface::func_void called");
        self.inner.func_void()
    }

    fn func_bool(
        &self,
        param_bool: bool,
    ) -> ApiFuture<'_, Result<bool, ApiError>> {
        tracing::info!("NoPropertiesInterface::func_bool called");
        self.inner.func_bool(param_bool)
    }

    fn publisher(&self) -> &NoPropertiesInterfacePublisher {
        self.inner.publisher()
    }
}
