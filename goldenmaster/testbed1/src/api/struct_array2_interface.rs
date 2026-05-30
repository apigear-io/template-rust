#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
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
