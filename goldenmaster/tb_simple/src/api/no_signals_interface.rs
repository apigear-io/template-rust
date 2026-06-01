use crate::api::{ApiError, ApiFuture};
use tokio::sync::{watch};

pub struct NoSignalsInterfacePublisher {
    pub prop_bool_changed: watch::Sender<bool>,
    pub prop_int_changed: watch::Sender<i32>,
}

impl Default for NoSignalsInterfacePublisher {
    fn default() -> Self {
        Self { prop_bool_changed: watch::channel(Default::default()).0, prop_int_changed: watch::channel(Default::default()).0 }
    }
}

pub trait NoSignalsInterfaceTrait: Send + Sync {
    fn func_void(&self) -> ApiFuture<'_, Result<(), ApiError>>;

    fn func_bool(
        &self,
        param_bool: bool,
    ) -> ApiFuture<'_, Result<bool, ApiError>>;

    /// Gets the value of the propBool property.
    fn prop_bool(&self) -> bool;
    /// Sets the value of the propBool property.
    fn set_prop_bool(
        &self,
        prop_bool: bool,
    );

    /// Gets the value of the propInt property.
    fn prop_int(&self) -> i32;
    /// Sets the value of the propInt property.
    fn set_prop_int(
        &self,
        prop_int: i32,
    );

    fn publisher(&self) -> &NoSignalsInterfacePublisher;
}

/// Async convenience wrappers for [`NoSignalsInterfaceTrait`] operations.
/// Provided for every implementor (including `dyn NoSignalsInterfaceTrait`) through a
/// blanket impl: call `obj.<op>_async(..).await` to get a `Result<_, ApiError>` directly.
pub trait NoSignalsInterfaceTraitAsync: NoSignalsInterfaceTrait {
    fn func_void_async(&self) -> impl std::future::Future<Output = Result<(), ApiError>> + Send {
        async move { self.func_void().await }
    }

    fn func_bool_async(
        &self,
        param_bool: bool,
    ) -> impl std::future::Future<Output = Result<bool, ApiError>> + Send {
        async move { self.func_bool(param_bool).await }
    }
}

impl<T: NoSignalsInterfaceTrait + ?Sized> NoSignalsInterfaceTraitAsync for T {}
