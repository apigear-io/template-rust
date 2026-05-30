use crate::api::simple_array_interface::SimpleArrayInterfaceTrait;
use apigear::{ApiError, ApiFuture};
use crate::api::simple_array_interface::SimpleArrayInterfacePublisher;
use parking_lot::RwLock;

pub struct SimpleArrayInterface {
    prop_bool: RwLock<Vec<bool>>,
    prop_int: RwLock<Vec<i32>>,
    prop_int32: RwLock<Vec<i32>>,
    prop_int64: RwLock<Vec<i64>>,
    prop_float: RwLock<Vec<f32>>,
    prop_float32: RwLock<Vec<f32>>,
    prop_float64: RwLock<Vec<f64>>,
    prop_string: RwLock<Vec<String>>,
    prop_read_only_string: RwLock<String>,
    publisher: SimpleArrayInterfacePublisher,
}

impl Default for SimpleArrayInterface {
    fn default() -> Self {
        Self { prop_bool: RwLock::new(Default::default()), prop_int: RwLock::new(Default::default()), prop_int32: RwLock::new(Default::default()), prop_int64: RwLock::new(Default::default()), prop_float: RwLock::new(Default::default()), prop_float32: RwLock::new(Default::default()), prop_float64: RwLock::new(Default::default()), prop_string: RwLock::new(Default::default()), prop_read_only_string: RwLock::new(Default::default()), publisher: Default::default() }
    }
}

impl SimpleArrayInterfaceTrait for SimpleArrayInterface {
    fn func_bool(
        &self,
        _param_bool: &[bool],
    ) -> ApiFuture<'_, Result<Vec<bool>, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn func_int(
        &self,
        _param_int: &[i32],
    ) -> ApiFuture<'_, Result<Vec<i32>, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn func_int32(
        &self,
        _param_int32: &[i32],
    ) -> ApiFuture<'_, Result<Vec<i32>, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn func_int64(
        &self,
        _param_int64: &[i64],
    ) -> ApiFuture<'_, Result<Vec<i64>, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn func_float(
        &self,
        _param_float: &[f32],
    ) -> ApiFuture<'_, Result<Vec<f32>, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn func_float32(
        &self,
        _param_float32: &[f32],
    ) -> ApiFuture<'_, Result<Vec<f32>, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn func_float64(
        &self,
        _param_float: &[f64],
    ) -> ApiFuture<'_, Result<Vec<f64>, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn func_string(
        &self,
        _param_string: &[String],
    ) -> ApiFuture<'_, Result<Vec<String>, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn prop_bool(&self) -> Vec<bool> {
        self.prop_bool.read().clone()
    }
    fn set_prop_bool(
        &self,
        prop_bool: &[bool],
    ) {
        let new_val = prop_bool.to_vec();
        let mut value = self.prop_bool.write();
        if *value == new_val {
            return;
        }
        *value = new_val.clone();
        let _ = self.publisher.prop_bool_changed.send(new_val);
    }

    fn prop_int(&self) -> Vec<i32> {
        self.prop_int.read().clone()
    }
    fn set_prop_int(
        &self,
        prop_int: &[i32],
    ) {
        let new_val = prop_int.to_vec();
        let mut value = self.prop_int.write();
        if *value == new_val {
            return;
        }
        *value = new_val.clone();
        let _ = self.publisher.prop_int_changed.send(new_val);
    }

    fn prop_int32(&self) -> Vec<i32> {
        self.prop_int32.read().clone()
    }
    fn set_prop_int32(
        &self,
        prop_int32: &[i32],
    ) {
        let new_val = prop_int32.to_vec();
        let mut value = self.prop_int32.write();
        if *value == new_val {
            return;
        }
        *value = new_val.clone();
        let _ = self.publisher.prop_int32_changed.send(new_val);
    }

    fn prop_int64(&self) -> Vec<i64> {
        self.prop_int64.read().clone()
    }
    fn set_prop_int64(
        &self,
        prop_int64: &[i64],
    ) {
        let new_val = prop_int64.to_vec();
        let mut value = self.prop_int64.write();
        if *value == new_val {
            return;
        }
        *value = new_val.clone();
        let _ = self.publisher.prop_int64_changed.send(new_val);
    }

    fn prop_float(&self) -> Vec<f32> {
        self.prop_float.read().clone()
    }
    fn set_prop_float(
        &self,
        prop_float: &[f32],
    ) {
        let new_val = prop_float.to_vec();
        let mut value = self.prop_float.write();
        if *value == new_val {
            return;
        }
        *value = new_val.clone();
        let _ = self.publisher.prop_float_changed.send(new_val);
    }

    fn prop_float32(&self) -> Vec<f32> {
        self.prop_float32.read().clone()
    }
    fn set_prop_float32(
        &self,
        prop_float32: &[f32],
    ) {
        let new_val = prop_float32.to_vec();
        let mut value = self.prop_float32.write();
        if *value == new_val {
            return;
        }
        *value = new_val.clone();
        let _ = self.publisher.prop_float32_changed.send(new_val);
    }

    fn prop_float64(&self) -> Vec<f64> {
        self.prop_float64.read().clone()
    }
    fn set_prop_float64(
        &self,
        prop_float64: &[f64],
    ) {
        let new_val = prop_float64.to_vec();
        let mut value = self.prop_float64.write();
        if *value == new_val {
            return;
        }
        *value = new_val.clone();
        let _ = self.publisher.prop_float64_changed.send(new_val);
    }

    fn prop_string(&self) -> Vec<String> {
        self.prop_string.read().clone()
    }
    fn set_prop_string(
        &self,
        prop_string: &[String],
    ) {
        let new_val = prop_string.to_vec();
        let mut value = self.prop_string.write();
        if *value == new_val {
            return;
        }
        *value = new_val.clone();
        let _ = self.publisher.prop_string_changed.send(new_val);
    }

    fn prop_read_only_string(&self) -> String {
        self.prop_read_only_string.read().clone()
    }

    fn publisher(&self) -> &SimpleArrayInterfacePublisher {
        &self.publisher
    }
}
