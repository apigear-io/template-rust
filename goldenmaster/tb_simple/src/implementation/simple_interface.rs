use crate::api::simple_interface::SimpleInterfaceTrait;
use apigear::{ApiError, ApiFuture};
use crate::api::simple_interface::SimpleInterfacePublisher;
use parking_lot::RwLock;

pub struct SimpleInterface {
    prop_bool: RwLock<bool>,
    prop_int: RwLock<i32>,
    prop_int32: RwLock<i32>,
    prop_int64: RwLock<i64>,
    prop_float: RwLock<f32>,
    prop_float32: RwLock<f32>,
    prop_float64: RwLock<f64>,
    prop_string: RwLock<String>,
    publisher: SimpleInterfacePublisher,
}

impl Default for SimpleInterface {
    fn default() -> Self {
        Self { prop_bool: RwLock::new(Default::default()), prop_int: RwLock::new(Default::default()), prop_int32: RwLock::new(Default::default()), prop_int64: RwLock::new(Default::default()), prop_float: RwLock::new(Default::default()), prop_float32: RwLock::new(Default::default()), prop_float64: RwLock::new(Default::default()), prop_string: RwLock::new(Default::default()), publisher: Default::default() }
    }
}

impl SimpleInterfaceTrait for SimpleInterface {
    fn func_no_return_value(
        &self,
        _param_bool: bool,
    ) -> ApiFuture<'_, Result<(), ApiError>> {
        Box::pin(async move { Ok(()) })
    }

    fn func_no_params(&self) -> ApiFuture<'_, Result<bool, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn func_bool(
        &self,
        _param_bool: bool,
    ) -> ApiFuture<'_, Result<bool, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn func_int(
        &self,
        _param_int: i32,
    ) -> ApiFuture<'_, Result<i32, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn func_int32(
        &self,
        _param_int32: i32,
    ) -> ApiFuture<'_, Result<i32, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn func_int64(
        &self,
        _param_int64: i64,
    ) -> ApiFuture<'_, Result<i64, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn func_float(
        &self,
        _param_float: f32,
    ) -> ApiFuture<'_, Result<f32, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn func_float32(
        &self,
        _param_float32: f32,
    ) -> ApiFuture<'_, Result<f32, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn func_float64(
        &self,
        _param_float: f64,
    ) -> ApiFuture<'_, Result<f64, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn func_string(
        &self,
        _param_string: &str,
    ) -> ApiFuture<'_, Result<String, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn prop_bool(&self) -> bool {
        *self.prop_bool.read()
    }
    fn set_prop_bool(
        &self,
        prop_bool: bool,
    ) {
        let mut value = self.prop_bool.write();
        if *value == prop_bool {
            return;
        }
        *value = prop_bool;
        let _ = self.publisher.prop_bool_changed.send(prop_bool);
    }

    fn prop_int(&self) -> i32 {
        *self.prop_int.read()
    }
    fn set_prop_int(
        &self,
        prop_int: i32,
    ) {
        let mut value = self.prop_int.write();
        if *value == prop_int {
            return;
        }
        *value = prop_int;
        let _ = self.publisher.prop_int_changed.send(prop_int);
    }

    fn prop_int32(&self) -> i32 {
        *self.prop_int32.read()
    }
    fn set_prop_int32(
        &self,
        prop_int32: i32,
    ) {
        let mut value = self.prop_int32.write();
        if *value == prop_int32 {
            return;
        }
        *value = prop_int32;
        let _ = self.publisher.prop_int32_changed.send(prop_int32);
    }

    fn prop_int64(&self) -> i64 {
        *self.prop_int64.read()
    }
    fn set_prop_int64(
        &self,
        prop_int64: i64,
    ) {
        let mut value = self.prop_int64.write();
        if *value == prop_int64 {
            return;
        }
        *value = prop_int64;
        let _ = self.publisher.prop_int64_changed.send(prop_int64);
    }

    fn prop_float(&self) -> f32 {
        *self.prop_float.read()
    }
    fn set_prop_float(
        &self,
        prop_float: f32,
    ) {
        let mut value = self.prop_float.write();
        if *value == prop_float {
            return;
        }
        *value = prop_float;
        let _ = self.publisher.prop_float_changed.send(prop_float);
    }

    fn prop_float32(&self) -> f32 {
        *self.prop_float32.read()
    }
    fn set_prop_float32(
        &self,
        prop_float32: f32,
    ) {
        let mut value = self.prop_float32.write();
        if *value == prop_float32 {
            return;
        }
        *value = prop_float32;
        let _ = self.publisher.prop_float32_changed.send(prop_float32);
    }

    fn prop_float64(&self) -> f64 {
        *self.prop_float64.read()
    }
    fn set_prop_float64(
        &self,
        prop_float64: f64,
    ) {
        let mut value = self.prop_float64.write();
        if *value == prop_float64 {
            return;
        }
        *value = prop_float64;
        let _ = self.publisher.prop_float64_changed.send(prop_float64);
    }

    fn prop_string(&self) -> String {
        self.prop_string.read().clone()
    }
    fn set_prop_string(
        &self,
        prop_string: &str,
    ) {
        let new_val = prop_string.to_string();
        let mut value = self.prop_string.write();
        if *value == new_val {
            return;
        }
        *value = new_val.clone();
        let _ = self.publisher.prop_string_changed.send(new_val);
    }

    fn publisher(&self) -> &SimpleInterfacePublisher {
        &self.publisher
    }
}
