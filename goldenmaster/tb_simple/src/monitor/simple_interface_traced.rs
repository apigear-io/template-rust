use apigear::{ApiError, ApiFuture};
use crate::api::simple_interface::SimpleInterfacePublisher;
use crate::api::simple_interface::SimpleInterfaceTrait;
use tracing;

/// Trace decorator for SimpleInterface.
/// Wraps any implementation and instruments all operations with tracing spans.
pub struct SimpleInterfaceTraced<T: SimpleInterfaceTrait> {
    inner: T,
}

impl<T: SimpleInterfaceTrait> SimpleInterfaceTraced<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: SimpleInterfaceTrait> SimpleInterfaceTrait for SimpleInterfaceTraced<T> {
    fn func_no_return_value(
        &self,
        param_bool: bool,
    ) -> ApiFuture<'_, Result<(), ApiError>> {
        tracing::info!("SimpleInterface::func_no_return_value called");
        self.inner.func_no_return_value(param_bool)
    }

    fn func_no_params(&self) -> ApiFuture<'_, Result<bool, ApiError>> {
        tracing::info!("SimpleInterface::func_no_params called");
        self.inner.func_no_params()
    }

    fn func_bool(
        &self,
        param_bool: bool,
    ) -> ApiFuture<'_, Result<bool, ApiError>> {
        tracing::info!("SimpleInterface::func_bool called");
        self.inner.func_bool(param_bool)
    }

    fn func_int(
        &self,
        param_int: i32,
    ) -> ApiFuture<'_, Result<i32, ApiError>> {
        tracing::info!("SimpleInterface::func_int called");
        self.inner.func_int(param_int)
    }

    fn func_int32(
        &self,
        param_int32: i32,
    ) -> ApiFuture<'_, Result<i32, ApiError>> {
        tracing::info!("SimpleInterface::func_int32 called");
        self.inner.func_int32(param_int32)
    }

    fn func_int64(
        &self,
        param_int64: i64,
    ) -> ApiFuture<'_, Result<i64, ApiError>> {
        tracing::info!("SimpleInterface::func_int64 called");
        self.inner.func_int64(param_int64)
    }

    fn func_float(
        &self,
        param_float: f32,
    ) -> ApiFuture<'_, Result<f32, ApiError>> {
        tracing::info!("SimpleInterface::func_float called");
        self.inner.func_float(param_float)
    }

    fn func_float32(
        &self,
        param_float32: f32,
    ) -> ApiFuture<'_, Result<f32, ApiError>> {
        tracing::info!("SimpleInterface::func_float32 called");
        self.inner.func_float32(param_float32)
    }

    fn func_float64(
        &self,
        param_float: f64,
    ) -> ApiFuture<'_, Result<f64, ApiError>> {
        tracing::info!("SimpleInterface::func_float64 called");
        self.inner.func_float64(param_float)
    }

    fn func_string(
        &self,
        param_string: &str,
    ) -> ApiFuture<'_, Result<String, ApiError>> {
        tracing::info!("SimpleInterface::func_string called");
        self.inner.func_string(param_string)
    }

    fn prop_bool(&self) -> bool {
        self.inner.prop_bool()
    }
    fn set_prop_bool(
        &self,
        prop_bool: bool,
    ) {
        tracing::info!("SimpleInterface::set_prop_bool called");
        self.inner.set_prop_bool(prop_bool);
    }

    fn prop_int(&self) -> i32 {
        self.inner.prop_int()
    }
    fn set_prop_int(
        &self,
        prop_int: i32,
    ) {
        tracing::info!("SimpleInterface::set_prop_int called");
        self.inner.set_prop_int(prop_int);
    }

    fn prop_int32(&self) -> i32 {
        self.inner.prop_int32()
    }
    fn set_prop_int32(
        &self,
        prop_int32: i32,
    ) {
        tracing::info!("SimpleInterface::set_prop_int32 called");
        self.inner.set_prop_int32(prop_int32);
    }

    fn prop_int64(&self) -> i64 {
        self.inner.prop_int64()
    }
    fn set_prop_int64(
        &self,
        prop_int64: i64,
    ) {
        tracing::info!("SimpleInterface::set_prop_int64 called");
        self.inner.set_prop_int64(prop_int64);
    }

    fn prop_float(&self) -> f32 {
        self.inner.prop_float()
    }
    fn set_prop_float(
        &self,
        prop_float: f32,
    ) {
        tracing::info!("SimpleInterface::set_prop_float called");
        self.inner.set_prop_float(prop_float);
    }

    fn prop_float32(&self) -> f32 {
        self.inner.prop_float32()
    }
    fn set_prop_float32(
        &self,
        prop_float32: f32,
    ) {
        tracing::info!("SimpleInterface::set_prop_float32 called");
        self.inner.set_prop_float32(prop_float32);
    }

    fn prop_float64(&self) -> f64 {
        self.inner.prop_float64()
    }
    fn set_prop_float64(
        &self,
        prop_float64: f64,
    ) {
        tracing::info!("SimpleInterface::set_prop_float64 called");
        self.inner.set_prop_float64(prop_float64);
    }

    fn prop_string(&self) -> String {
        self.inner.prop_string()
    }
    fn set_prop_string(
        &self,
        prop_string: &str,
    ) {
        tracing::info!("SimpleInterface::set_prop_string called");
        self.inner.set_prop_string(prop_string);
    }

    fn publisher(&self) -> &SimpleInterfacePublisher {
        self.inner.publisher()
    }
}
