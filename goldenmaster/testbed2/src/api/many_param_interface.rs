#[allow(unused_imports)]
use crate::api::data_structs::*;
use crate::api::{ApiError, ApiFuture};
use tokio::sync::{watch, broadcast};

pub struct ManyParamInterfacePublisher {
    pub prop1_changed: watch::Sender<i32>,
    pub prop2_changed: watch::Sender<i32>,
    pub prop3_changed: watch::Sender<i32>,
    pub prop4_changed: watch::Sender<i32>,
    pub sig1: broadcast::Sender<(i32,)>,
    pub sig2: broadcast::Sender<(i32, i32)>,
    pub sig3: broadcast::Sender<(i32, i32, i32)>,
    pub sig4: broadcast::Sender<(i32, i32, i32, i32)>,
}

impl Default for ManyParamInterfacePublisher {
    fn default() -> Self {
        Self { prop1_changed: watch::channel(Default::default()).0, prop2_changed: watch::channel(Default::default()).0, prop3_changed: watch::channel(Default::default()).0, prop4_changed: watch::channel(Default::default()).0, sig1: broadcast::Sender::new(16), sig2: broadcast::Sender::new(16), sig3: broadcast::Sender::new(16), sig4: broadcast::Sender::new(16) }
    }
}

pub trait ManyParamInterfaceTrait: Send + Sync {
    fn func1(
        &self,
        param1: i32,
    ) -> ApiFuture<'_, Result<i32, ApiError>>;

    fn func2(
        &self,
        param1: i32,
        param2: i32,
    ) -> ApiFuture<'_, Result<i32, ApiError>>;

    fn func3(
        &self,
        param1: i32,
        param2: i32,
        param3: i32,
    ) -> ApiFuture<'_, Result<i32, ApiError>>;

    fn func4(
        &self,
        param1: i32,
        param2: i32,
        param3: i32,
        param4: i32,
    ) -> ApiFuture<'_, Result<i32, ApiError>>;

    /// Gets the value of the prop1 property.
    fn prop1(&self) -> i32;
    /// Sets the value of the prop1 property.
    fn set_prop1(
        &self,
        prop1: i32,
    );

    /// Gets the value of the prop2 property.
    fn prop2(&self) -> i32;
    /// Sets the value of the prop2 property.
    fn set_prop2(
        &self,
        prop2: i32,
    );

    /// Gets the value of the prop3 property.
    fn prop3(&self) -> i32;
    /// Sets the value of the prop3 property.
    fn set_prop3(
        &self,
        prop3: i32,
    );

    /// Gets the value of the prop4 property.
    fn prop4(&self) -> i32;
    /// Sets the value of the prop4 property.
    fn set_prop4(
        &self,
        prop4: i32,
    );

    fn publisher(&self) -> &ManyParamInterfacePublisher;
}

/// Async convenience wrappers for [`ManyParamInterfaceTrait`] operations.
/// Provided for every implementor (including `dyn ManyParamInterfaceTrait`) through a
/// blanket impl: call `obj.<op>_async(..).await` to get a `Result<_, ApiError>` directly.
pub trait ManyParamInterfaceTraitAsync: ManyParamInterfaceTrait {
    fn func1_async(
        &self,
        param1: i32,
    ) -> impl std::future::Future<Output = Result<i32, ApiError>> + Send {
        async move { self.func1(param1).await }
    }

    fn func2_async(
        &self,
        param1: i32,
        param2: i32,
    ) -> impl std::future::Future<Output = Result<i32, ApiError>> + Send {
        async move { self.func2(param1, param2).await }
    }

    fn func3_async(
        &self,
        param1: i32,
        param2: i32,
        param3: i32,
    ) -> impl std::future::Future<Output = Result<i32, ApiError>> + Send {
        async move { self.func3(param1, param2, param3).await }
    }

    fn func4_async(
        &self,
        param1: i32,
        param2: i32,
        param3: i32,
        param4: i32,
    ) -> impl std::future::Future<Output = Result<i32, ApiError>> + Send {
        async move { self.func4(param1, param2, param3, param4).await }
    }
}

impl<T: ManyParamInterfaceTrait + ?Sized> ManyParamInterfaceTraitAsync for T {}
