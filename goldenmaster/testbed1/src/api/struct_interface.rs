#[allow(unused_imports)]
use crate::api::data_structs::*;
use crate::api::{ApiError, ApiFuture};
use tokio::sync::{watch, broadcast};

pub struct StructInterfacePublisher {
    pub prop_bool_changed: watch::Sender<StructBool>,
    pub prop_int_changed: watch::Sender<StructInt>,
    pub prop_float_changed: watch::Sender<StructFloat>,
    pub prop_string_changed: watch::Sender<StructString>,
    pub sig_bool: broadcast::Sender<(StructBool,)>,
    pub sig_int: broadcast::Sender<(StructInt,)>,
    pub sig_float: broadcast::Sender<(StructFloat,)>,
    pub sig_string: broadcast::Sender<(StructString,)>,
}

impl Default for StructInterfacePublisher {
    fn default() -> Self {
        Self { prop_bool_changed: watch::channel(Default::default()).0, prop_int_changed: watch::channel(Default::default()).0, prop_float_changed: watch::channel(Default::default()).0, prop_string_changed: watch::channel(Default::default()).0, sig_bool: broadcast::Sender::new(16), sig_int: broadcast::Sender::new(16), sig_float: broadcast::Sender::new(16), sig_string: broadcast::Sender::new(16) }
    }
}

pub trait StructInterfaceTrait: Send + Sync {
    fn func_bool(
        &self,
        param_bool: &StructBool,
    ) -> ApiFuture<'_, Result<StructBool, ApiError>>;

    fn func_int(
        &self,
        param_int: &StructInt,
    ) -> ApiFuture<'_, Result<StructInt, ApiError>>;

    fn func_float(
        &self,
        param_float: &StructFloat,
    ) -> ApiFuture<'_, Result<StructFloat, ApiError>>;

    fn func_string(
        &self,
        param_string: &StructString,
    ) -> ApiFuture<'_, Result<StructString, ApiError>>;

    /// Gets the value of the propBool property.
    fn prop_bool(&self) -> StructBool;
    /// Sets the value of the propBool property.
    fn set_prop_bool(
        &self,
        prop_bool: &StructBool,
    );

    /// Gets the value of the propInt property.
    fn prop_int(&self) -> StructInt;
    /// Sets the value of the propInt property.
    fn set_prop_int(
        &self,
        prop_int: &StructInt,
    );

    /// Gets the value of the propFloat property.
    fn prop_float(&self) -> StructFloat;
    /// Sets the value of the propFloat property.
    fn set_prop_float(
        &self,
        prop_float: &StructFloat,
    );

    /// Gets the value of the propString property.
    fn prop_string(&self) -> StructString;
    /// Sets the value of the propString property.
    fn set_prop_string(
        &self,
        prop_string: &StructString,
    );

    fn publisher(&self) -> &StructInterfacePublisher;
}

/// Async convenience wrappers for [`StructInterfaceTrait`] operations.
/// Provided for every implementor (including `dyn StructInterfaceTrait`) through a
/// blanket impl: call `obj.<op>_async(..).await` to get a `Result<_, ApiError>` directly.
pub trait StructInterfaceTraitAsync: StructInterfaceTrait {
    fn func_bool_async(
        &self,
        param_bool: &StructBool,
    ) -> impl std::future::Future<Output = Result<StructBool, ApiError>> + Send {
        async move { self.func_bool(param_bool).await }
    }

    fn func_int_async(
        &self,
        param_int: &StructInt,
    ) -> impl std::future::Future<Output = Result<StructInt, ApiError>> + Send {
        async move { self.func_int(param_int).await }
    }

    fn func_float_async(
        &self,
        param_float: &StructFloat,
    ) -> impl std::future::Future<Output = Result<StructFloat, ApiError>> + Send {
        async move { self.func_float(param_float).await }
    }

    fn func_string_async(
        &self,
        param_string: &StructString,
    ) -> impl std::future::Future<Output = Result<StructString, ApiError>> + Send {
        async move { self.func_string(param_string).await }
    }
}

impl<T: StructInterfaceTrait + ?Sized> StructInterfaceTraitAsync for T {}
