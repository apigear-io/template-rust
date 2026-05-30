use crate::api::no_properties_interface::NoPropertiesInterfaceTrait;
use apigear::{ApiError, ApiFuture};
use crate::api::no_properties_interface::NoPropertiesInterfacePublisher;

#[derive(Default)]
pub struct NoPropertiesInterface {
    publisher: NoPropertiesInterfacePublisher,
}

impl NoPropertiesInterfaceTrait for NoPropertiesInterface {
    fn func_void(&self) -> ApiFuture<'_, Result<(), ApiError>> {
        Box::pin(async move { Ok(()) })
    }

    fn func_bool(
        &self,
        _param_bool: bool,
    ) -> ApiFuture<'_, Result<bool, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn publisher(&self) -> &NoPropertiesInterfacePublisher {
        &self.publisher
    }
}
