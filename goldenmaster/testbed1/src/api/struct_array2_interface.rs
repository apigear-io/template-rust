#[allow(unused_imports)]
use crate::api::data_structs::*;
use crate::api::{ApiError, ApiFuture};
use tokio::sync::{watch, broadcast};

pub struct StructArray2InterfacePublisher {
    pub prop_bool_changed: watch::Sender<StructBoolWithArray>,
    pub prop_int_changed: watch::Sender<StructIntWithArray>,
    pub prop_float_changed: watch::Sender<StructFloatWithArray>,
    pub prop_string_changed: watch::Sender<StructStringWithArray>,
    pub prop_enum_changed: watch::Sender<StructEnumWithArray>,
    pub sig_bool: broadcast::Sender<(StructBoolWithArray,)>,
    pub sig_int: broadcast::Sender<(StructIntWithArray,)>,
    pub sig_float: broadcast::Sender<(StructFloatWithArray,)>,
    pub sig_string: broadcast::Sender<(StructStringWithArray,)>,
}

impl Default for StructArray2InterfacePublisher {
    fn default() -> Self {
        Self { prop_bool_changed: watch::channel(Default::default()).0, prop_int_changed: watch::channel(Default::default()).0, prop_float_changed: watch::channel(Default::default()).0, prop_string_changed: watch::channel(Default::default()).0, prop_enum_changed: watch::channel(Default::default()).0, sig_bool: broadcast::Sender::new(16), sig_int: broadcast::Sender::new(16), sig_float: broadcast::Sender::new(16), sig_string: broadcast::Sender::new(16) }
    }
}

pub trait StructArray2InterfaceTrait: Send + Sync {
    fn func_bool(
        &self,
        param_bool: &StructBoolWithArray,
    ) -> ApiFuture<'_, Result<Vec<StructBool>, ApiError>>;

    fn func_int(
        &self,
        param_int: &StructIntWithArray,
    ) -> ApiFuture<'_, Result<Vec<StructInt>, ApiError>>;

    fn func_float(
        &self,
        param_float: &StructFloatWithArray,
    ) -> ApiFuture<'_, Result<Vec<StructFloat>, ApiError>>;

    fn func_string(
        &self,
        param_string: &StructStringWithArray,
    ) -> ApiFuture<'_, Result<Vec<StructString>, ApiError>>;

    fn func_enum(
        &self,
        param_enum: &StructEnumWithArray,
    ) -> ApiFuture<'_, Result<Vec<Enum0Enum>, ApiError>>;

    /// Gets the value of the propBool property.
    fn prop_bool(&self) -> StructBoolWithArray;
    /// Sets the value of the propBool property.
    fn set_prop_bool(
        &self,
        prop_bool: &StructBoolWithArray,
    );

    /// Gets the value of the propInt property.
    fn prop_int(&self) -> StructIntWithArray;
    /// Sets the value of the propInt property.
    fn set_prop_int(
        &self,
        prop_int: &StructIntWithArray,
    );

    /// Gets the value of the propFloat property.
    fn prop_float(&self) -> StructFloatWithArray;
    /// Sets the value of the propFloat property.
    fn set_prop_float(
        &self,
        prop_float: &StructFloatWithArray,
    );

    /// Gets the value of the propString property.
    fn prop_string(&self) -> StructStringWithArray;
    /// Sets the value of the propString property.
    fn set_prop_string(
        &self,
        prop_string: &StructStringWithArray,
    );

    /// Gets the value of the propEnum property.
    fn prop_enum(&self) -> StructEnumWithArray;
    /// Sets the value of the propEnum property.
    fn set_prop_enum(
        &self,
        prop_enum: &StructEnumWithArray,
    );

    fn publisher(&self) -> &StructArray2InterfacePublisher;
}

/// Async convenience wrappers for [`StructArray2InterfaceTrait`] operations.
/// Provided for every implementor (including `dyn StructArray2InterfaceTrait`) through a
/// blanket impl: call `obj.<op>_async(..).await` to get a `Result<_, ApiError>` directly.
pub trait StructArray2InterfaceTraitAsync: StructArray2InterfaceTrait {
    fn func_bool_async(
        &self,
        param_bool: &StructBoolWithArray,
    ) -> impl std::future::Future<Output = Result<Vec<StructBool>, ApiError>> + Send {
        async move { self.func_bool(param_bool).await }
    }

    fn func_int_async(
        &self,
        param_int: &StructIntWithArray,
    ) -> impl std::future::Future<Output = Result<Vec<StructInt>, ApiError>> + Send {
        async move { self.func_int(param_int).await }
    }

    fn func_float_async(
        &self,
        param_float: &StructFloatWithArray,
    ) -> impl std::future::Future<Output = Result<Vec<StructFloat>, ApiError>> + Send {
        async move { self.func_float(param_float).await }
    }

    fn func_string_async(
        &self,
        param_string: &StructStringWithArray,
    ) -> impl std::future::Future<Output = Result<Vec<StructString>, ApiError>> + Send {
        async move { self.func_string(param_string).await }
    }

    fn func_enum_async(
        &self,
        param_enum: &StructEnumWithArray,
    ) -> impl std::future::Future<Output = Result<Vec<Enum0Enum>, ApiError>> + Send {
        async move { self.func_enum(param_enum).await }
    }
}

impl<T: StructArray2InterfaceTrait + ?Sized> StructArray2InterfaceTraitAsync for T {}
