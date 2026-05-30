#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
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
