use crate::api::void_interface::VoidInterfaceTrait;
use apigear::{ApiError, ApiFuture};
use crate::api::void_interface::VoidInterfacePublisher;

#[derive(Default)]
pub struct VoidInterface {
    publisher: VoidInterfacePublisher,
}

impl VoidInterfaceTrait for VoidInterface {
    fn func_void(&self) -> ApiFuture<'_, Result<(), ApiError>> {
        Box::pin(async move { Ok(()) })
    }

    fn publisher(&self) -> &VoidInterfacePublisher {
        &self.publisher
    }
}
