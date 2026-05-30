#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use tokio::sync::{watch, broadcast};

pub struct StructArrayInterfacePublisher {
    pub prop_bool_changed: watch::Sender<Vec<StructBool>>,
    pub prop_int_changed: watch::Sender<Vec<StructInt>>,
    pub prop_float_changed: watch::Sender<Vec<StructFloat>>,
    pub prop_string_changed: watch::Sender<Vec<StructString>>,
    pub prop_enum_changed: watch::Sender<Vec<Enum0Enum>>,
    pub sig_bool: broadcast::Sender<(Vec<StructBool>,)>,
    pub sig_int: broadcast::Sender<(Vec<StructInt>,)>,
    pub sig_float: broadcast::Sender<(Vec<StructFloat>,)>,
    pub sig_string: broadcast::Sender<(Vec<StructString>,)>,
    pub sig_enum: broadcast::Sender<(Vec<Enum0Enum>,)>,
}

impl Default for StructArrayInterfacePublisher {
    fn default() -> Self {
        Self { prop_bool_changed: watch::channel(Default::default()).0, prop_int_changed: watch::channel(Default::default()).0, prop_float_changed: watch::channel(Default::default()).0, prop_string_changed: watch::channel(Default::default()).0, prop_enum_changed: watch::channel(Default::default()).0, sig_bool: broadcast::Sender::new(16), sig_int: broadcast::Sender::new(16), sig_float: broadcast::Sender::new(16), sig_string: broadcast::Sender::new(16), sig_enum: broadcast::Sender::new(16) }
    }
}

pub trait StructArrayInterfaceTrait: Send + Sync {
    fn func_bool(
        &self,
        param_bool: &[StructBool],
    ) -> ApiFuture<'_, Result<Vec<StructBool>, ApiError>>;

    fn func_int(
        &self,
        param_int: &[StructInt],
    ) -> ApiFuture<'_, Result<Vec<StructInt>, ApiError>>;

    fn func_float(
        &self,
        param_float: &[StructFloat],
    ) -> ApiFuture<'_, Result<Vec<StructFloat>, ApiError>>;

    fn func_string(
        &self,
        param_string: &[StructString],
    ) -> ApiFuture<'_, Result<Vec<StructString>, ApiError>>;

    fn func_enum(
        &self,
        param_enum: &[Enum0Enum],
    ) -> ApiFuture<'_, Result<Vec<Enum0Enum>, ApiError>>;

    /// Gets the value of the propBool property.
    fn prop_bool(&self) -> Vec<StructBool>;
    /// Sets the value of the propBool property.
    fn set_prop_bool(
        &self,
        prop_bool: &[StructBool],
    );

    /// Gets the value of the propInt property.
    fn prop_int(&self) -> Vec<StructInt>;
    /// Sets the value of the propInt property.
    fn set_prop_int(
        &self,
        prop_int: &[StructInt],
    );

    /// Gets the value of the propFloat property.
    fn prop_float(&self) -> Vec<StructFloat>;
    /// Sets the value of the propFloat property.
    fn set_prop_float(
        &self,
        prop_float: &[StructFloat],
    );

    /// Gets the value of the propString property.
    fn prop_string(&self) -> Vec<StructString>;
    /// Sets the value of the propString property.
    fn set_prop_string(
        &self,
        prop_string: &[StructString],
    );

    /// Gets the value of the propEnum property.
    fn prop_enum(&self) -> Vec<Enum0Enum>;
    /// Sets the value of the propEnum property.
    fn set_prop_enum(
        &self,
        prop_enum: &[Enum0Enum],
    );

    fn publisher(&self) -> &StructArrayInterfacePublisher;
}
