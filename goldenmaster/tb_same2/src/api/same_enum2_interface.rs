#[allow(unused_imports)]
use crate::api::data_structs::*;
use crate::api::{ApiError, ApiFuture};
use tokio::sync::{watch, broadcast};

pub struct SameEnum2InterfacePublisher {
    pub prop1_changed: watch::Sender<Enum1Enum>,
    pub prop2_changed: watch::Sender<Enum2Enum>,
    pub sig1: broadcast::Sender<(Enum1Enum,)>,
    pub sig2: broadcast::Sender<(Enum1Enum, Enum2Enum)>,
}

impl Default for SameEnum2InterfacePublisher {
    fn default() -> Self {
        Self { prop1_changed: watch::channel(Default::default()).0, prop2_changed: watch::channel(Default::default()).0, sig1: broadcast::Sender::new(16), sig2: broadcast::Sender::new(16) }
    }
}

pub trait SameEnum2InterfaceTrait: Send + Sync {
    fn func1(
        &self,
        param1: Enum1Enum,
    ) -> ApiFuture<'_, Result<Enum1Enum, ApiError>>;

    fn func2(
        &self,
        param1: Enum1Enum,
        param2: Enum2Enum,
    ) -> ApiFuture<'_, Result<Enum1Enum, ApiError>>;

    /// Gets the value of the prop1 property.
    fn prop1(&self) -> Enum1Enum;
    /// Sets the value of the prop1 property.
    fn set_prop1(
        &self,
        prop1: Enum1Enum,
    );

    /// Gets the value of the prop2 property.
    fn prop2(&self) -> Enum2Enum;
    /// Sets the value of the prop2 property.
    fn set_prop2(
        &self,
        prop2: Enum2Enum,
    );

    fn publisher(&self) -> &SameEnum2InterfacePublisher;
}

/// Async convenience wrappers for [`SameEnum2InterfaceTrait`] operations.
/// Provided for every implementor (including `dyn SameEnum2InterfaceTrait`) through a
/// blanket impl: call `obj.<op>_async(..).await` to get a `Result<_, ApiError>` directly.
pub trait SameEnum2InterfaceTraitAsync: SameEnum2InterfaceTrait {
    fn func1_async(
        &self,
        param1: Enum1Enum,
    ) -> impl std::future::Future<Output = Result<Enum1Enum, ApiError>> + Send {
        async move { self.func1(param1).await }
    }

    fn func2_async(
        &self,
        param1: Enum1Enum,
        param2: Enum2Enum,
    ) -> impl std::future::Future<Output = Result<Enum1Enum, ApiError>> + Send {
        async move { self.func2(param1, param2).await }
    }
}

impl<T: SameEnum2InterfaceTrait + ?Sized> SameEnum2InterfaceTraitAsync for T {}
