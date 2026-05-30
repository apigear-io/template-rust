use apigear::{ApiError, ApiFuture};
use tokio::sync::{watch, broadcast};

pub struct SimpleInterfacePublisher {
    pub prop_bool_changed: watch::Sender<bool>,
    pub prop_int_changed: watch::Sender<i32>,
    pub prop_int32_changed: watch::Sender<i32>,
    pub prop_int64_changed: watch::Sender<i64>,
    pub prop_float_changed: watch::Sender<f32>,
    pub prop_float32_changed: watch::Sender<f32>,
    pub prop_float64_changed: watch::Sender<f64>,
    pub prop_string_changed: watch::Sender<String>,
    pub sig_bool: broadcast::Sender<(bool,)>,
    pub sig_int: broadcast::Sender<(i32,)>,
    pub sig_int32: broadcast::Sender<(i32,)>,
    pub sig_int64: broadcast::Sender<(i64,)>,
    pub sig_float: broadcast::Sender<(f32,)>,
    pub sig_float32: broadcast::Sender<(f32,)>,
    pub sig_float64: broadcast::Sender<(f64,)>,
    pub sig_string: broadcast::Sender<(String,)>,
}

impl Default for SimpleInterfacePublisher {
    fn default() -> Self {
        Self { prop_bool_changed: watch::channel(Default::default()).0, prop_int_changed: watch::channel(Default::default()).0, prop_int32_changed: watch::channel(Default::default()).0, prop_int64_changed: watch::channel(Default::default()).0, prop_float_changed: watch::channel(Default::default()).0, prop_float32_changed: watch::channel(Default::default()).0, prop_float64_changed: watch::channel(Default::default()).0, prop_string_changed: watch::channel(Default::default()).0, sig_bool: broadcast::Sender::new(16), sig_int: broadcast::Sender::new(16), sig_int32: broadcast::Sender::new(16), sig_int64: broadcast::Sender::new(16), sig_float: broadcast::Sender::new(16), sig_float32: broadcast::Sender::new(16), sig_float64: broadcast::Sender::new(16), sig_string: broadcast::Sender::new(16) }
    }
}

pub trait SimpleInterfaceTrait: Send + Sync {
    fn func_no_return_value(
        &self,
        param_bool: bool,
    ) -> ApiFuture<'_, Result<(), ApiError>>;

    fn func_no_params(&self) -> ApiFuture<'_, Result<bool, ApiError>>;

    fn func_bool(
        &self,
        param_bool: bool,
    ) -> ApiFuture<'_, Result<bool, ApiError>>;

    fn func_int(
        &self,
        param_int: i32,
    ) -> ApiFuture<'_, Result<i32, ApiError>>;

    fn func_int32(
        &self,
        param_int32: i32,
    ) -> ApiFuture<'_, Result<i32, ApiError>>;

    fn func_int64(
        &self,
        param_int64: i64,
    ) -> ApiFuture<'_, Result<i64, ApiError>>;

    fn func_float(
        &self,
        param_float: f32,
    ) -> ApiFuture<'_, Result<f32, ApiError>>;

    fn func_float32(
        &self,
        param_float32: f32,
    ) -> ApiFuture<'_, Result<f32, ApiError>>;

    fn func_float64(
        &self,
        param_float: f64,
    ) -> ApiFuture<'_, Result<f64, ApiError>>;

    fn func_string(
        &self,
        param_string: &str,
    ) -> ApiFuture<'_, Result<String, ApiError>>;

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

    /// Gets the value of the propInt32 property.
    fn prop_int32(&self) -> i32;
    /// Sets the value of the propInt32 property.
    fn set_prop_int32(
        &self,
        prop_int32: i32,
    );

    /// Gets the value of the propInt64 property.
    fn prop_int64(&self) -> i64;
    /// Sets the value of the propInt64 property.
    fn set_prop_int64(
        &self,
        prop_int64: i64,
    );

    /// Gets the value of the propFloat property.
    fn prop_float(&self) -> f32;
    /// Sets the value of the propFloat property.
    fn set_prop_float(
        &self,
        prop_float: f32,
    );

    /// Gets the value of the propFloat32 property.
    fn prop_float32(&self) -> f32;
    /// Sets the value of the propFloat32 property.
    fn set_prop_float32(
        &self,
        prop_float32: f32,
    );

    /// Gets the value of the propFloat64 property.
    fn prop_float64(&self) -> f64;
    /// Sets the value of the propFloat64 property.
    fn set_prop_float64(
        &self,
        prop_float64: f64,
    );

    /// Gets the value of the propString property.
    fn prop_string(&self) -> String;
    /// Sets the value of the propString property.
    fn set_prop_string(
        &self,
        prop_string: &str,
    );

    fn publisher(&self) -> &SimpleInterfacePublisher;
}
