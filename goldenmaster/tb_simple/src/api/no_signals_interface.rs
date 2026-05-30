use apigear::{ApiError, ApiFuture};
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
