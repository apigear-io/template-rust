#[allow(unused_imports)]
use crate::api::data_structs::*;
use crate::api::{ApiError, ApiFuture};
use tokio::sync::{watch, broadcast};

pub struct NestedStruct1InterfacePublisher {
    pub prop1_changed: watch::Sender<NestedStruct1>,
    pub sig1: broadcast::Sender<(NestedStruct1,)>,
}

impl Default for NestedStruct1InterfacePublisher {
    fn default() -> Self {
        Self { prop1_changed: watch::channel(Default::default()).0, sig1: broadcast::Sender::new(16) }
    }
}

pub trait NestedStruct1InterfaceTrait: Send + Sync {
    fn func_no_return_value(
        &self,
        param1: &NestedStruct1,
    ) -> ApiFuture<'_, Result<(), ApiError>>;

    fn func_no_params(&self) -> ApiFuture<'_, Result<NestedStruct1, ApiError>>;

    fn func1(
        &self,
        param1: &NestedStruct1,
    ) -> ApiFuture<'_, Result<NestedStruct1, ApiError>>;

    /// Gets the value of the prop1 property.
    fn prop1(&self) -> NestedStruct1;
    /// Sets the value of the prop1 property.
    fn set_prop1(
        &self,
        prop1: &NestedStruct1,
    );

    fn publisher(&self) -> &NestedStruct1InterfacePublisher;
}

/// Async convenience wrappers for [`NestedStruct1InterfaceTrait`] operations.
/// Provided for every implementor (including `dyn NestedStruct1InterfaceTrait`) through a
/// blanket impl: call `obj.<op>_async(..).await` to get a `Result<_, ApiError>` directly.
pub trait NestedStruct1InterfaceTraitAsync: NestedStruct1InterfaceTrait {
    fn func_no_return_value_async(
        &self,
        param1: &NestedStruct1,
    ) -> impl std::future::Future<Output = Result<(), ApiError>> + Send {
        async move { self.func_no_return_value(param1).await }
    }

    fn func_no_params_async(&self) -> impl std::future::Future<Output = Result<NestedStruct1, ApiError>> + Send {
        async move { self.func_no_params().await }
    }

    fn func1_async(
        &self,
        param1: &NestedStruct1,
    ) -> impl std::future::Future<Output = Result<NestedStruct1, ApiError>> + Send {
        async move { self.func1(param1).await }
    }
}

impl<T: NestedStruct1InterfaceTrait + ?Sized> NestedStruct1InterfaceTraitAsync for T {}
