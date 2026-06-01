#[allow(unused_imports)]
use crate::api::data_structs::*;
use crate::api::{ApiError, ApiFuture};
use tokio::sync::{watch, broadcast};

pub struct SameStruct2InterfacePublisher {
    pub prop1_changed: watch::Sender<Struct2>,
    pub prop2_changed: watch::Sender<Struct2>,
    pub sig1: broadcast::Sender<(Struct1,)>,
    pub sig2: broadcast::Sender<(Struct1, Struct2)>,
}

impl Default for SameStruct2InterfacePublisher {
    fn default() -> Self {
        Self { prop1_changed: watch::channel(Default::default()).0, prop2_changed: watch::channel(Default::default()).0, sig1: broadcast::Sender::new(16), sig2: broadcast::Sender::new(16) }
    }
}

pub trait SameStruct2InterfaceTrait: Send + Sync {
    fn func1(
        &self,
        param1: &Struct1,
    ) -> ApiFuture<'_, Result<Struct1, ApiError>>;

    fn func2(
        &self,
        param1: &Struct1,
        param2: &Struct2,
    ) -> ApiFuture<'_, Result<Struct1, ApiError>>;

    /// Gets the value of the prop1 property.
    fn prop1(&self) -> Struct2;
    /// Sets the value of the prop1 property.
    fn set_prop1(
        &self,
        prop1: &Struct2,
    );

    /// Gets the value of the prop2 property.
    fn prop2(&self) -> Struct2;
    /// Sets the value of the prop2 property.
    fn set_prop2(
        &self,
        prop2: &Struct2,
    );

    fn publisher(&self) -> &SameStruct2InterfacePublisher;
}

/// Async convenience wrappers for [`SameStruct2InterfaceTrait`] operations.
/// Provided for every implementor (including `dyn SameStruct2InterfaceTrait`) through a
/// blanket impl: call `obj.<op>_async(..).await` to get a `Result<_, ApiError>` directly.
pub trait SameStruct2InterfaceTraitAsync: SameStruct2InterfaceTrait {
    fn func1_async(
        &self,
        param1: &Struct1,
    ) -> impl std::future::Future<Output = Result<Struct1, ApiError>> + Send {
        async move { self.func1(param1).await }
    }

    fn func2_async(
        &self,
        param1: &Struct1,
        param2: &Struct2,
    ) -> impl std::future::Future<Output = Result<Struct1, ApiError>> + Send {
        async move { self.func2(param1, param2).await }
    }
}

impl<T: SameStruct2InterfaceTrait + ?Sized> SameStruct2InterfaceTraitAsync for T {}
