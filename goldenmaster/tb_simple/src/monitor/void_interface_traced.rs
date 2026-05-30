use apigear::{ApiError, ApiFuture};
use crate::api::void_interface::VoidInterfacePublisher;
use crate::api::void_interface::VoidInterfaceTrait;
use tracing;

/// Trace decorator for VoidInterface.
/// Wraps any implementation and instruments all operations with tracing spans.
pub struct VoidInterfaceTraced<T: VoidInterfaceTrait> {
    inner: T,
}

impl<T: VoidInterfaceTrait> VoidInterfaceTraced<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: VoidInterfaceTrait> VoidInterfaceTrait for VoidInterfaceTraced<T> {
    fn func_void(&self) -> ApiFuture<'_, Result<(), ApiError>> {
        tracing::info!("VoidInterface::func_void called");
        self.inner.func_void()
    }

    fn publisher(&self) -> &VoidInterfacePublisher {
        self.inner.publisher()
    }
}
