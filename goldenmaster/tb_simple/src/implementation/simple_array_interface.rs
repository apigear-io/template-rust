use crate::api::simple_array_interface::SimpleArrayInterfaceTrait;
use async_trait::async_trait;
use crate::api::simple_array_interface::SimpleArrayInterfaceSignalHandler;
use signals2::*;

#[derive(Default, Clone)]
pub struct SimpleArrayInterface {
    prop_bool: Vec<bool>,
    prop_int: Vec<i32>,
    prop_int32: Vec<i32>,
    prop_int64: Vec<i64>,
    prop_float: Vec<f32>,
    prop_float32: Vec<f32>,
    prop_float64: Vec<f64>,
    prop_string: Vec<String>,
    _signal_handler: SimpleArrayInterfaceSignalHandler,
}

#[async_trait]
impl SimpleArrayInterfaceTrait for SimpleArrayInterface {
    fn func_bool(
        &mut self,
        _param_bool: &[bool],
    ) -> Vec<bool> {
        Default::default()
    }
    /// Asynchronous version of [func_bool](SimpleArrayInterface::func_bool)
    /// returns future of type [`Vec<bool>`] which is set once the function has completed
    async fn func_bool_async(
        &mut self,
        param_bool: &[bool],
    ) -> Result<Vec<bool>, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func_bool(param_bool))
    }

    fn func_int(
        &mut self,
        _param_int: &[i32],
    ) -> Vec<i32> {
        Default::default()
    }
    /// Asynchronous version of [func_int](SimpleArrayInterface::func_int)
    /// returns future of type [`Vec<i32>`] which is set once the function has completed
    async fn func_int_async(
        &mut self,
        param_int: &[i32],
    ) -> Result<Vec<i32>, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func_int(param_int))
    }

    fn func_int32(
        &mut self,
        _param_int32: &[i32],
    ) -> Vec<i32> {
        Default::default()
    }
    /// Asynchronous version of [func_int32](SimpleArrayInterface::func_int32)
    /// returns future of type [`Vec<i32>`] which is set once the function has completed
    async fn func_int32_async(
        &mut self,
        param_int32: &[i32],
    ) -> Result<Vec<i32>, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func_int32(param_int32))
    }

    fn func_int64(
        &mut self,
        _param_int64: &[i64],
    ) -> Vec<i64> {
        Default::default()
    }
    /// Asynchronous version of [func_int64](SimpleArrayInterface::func_int64)
    /// returns future of type [`Vec<i64>`] which is set once the function has completed
    async fn func_int64_async(
        &mut self,
        param_int64: &[i64],
    ) -> Result<Vec<i64>, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func_int64(param_int64))
    }

    fn func_float(
        &mut self,
        _param_float: &[f32],
    ) -> Vec<f32> {
        Default::default()
    }
    /// Asynchronous version of [func_float](SimpleArrayInterface::func_float)
    /// returns future of type [`Vec<f32>`] which is set once the function has completed
    async fn func_float_async(
        &mut self,
        param_float: &[f32],
    ) -> Result<Vec<f32>, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func_float(param_float))
    }

    fn func_float32(
        &mut self,
        _param_float32: &[f32],
    ) -> Vec<f32> {
        Default::default()
    }
    /// Asynchronous version of [func_float32](SimpleArrayInterface::func_float32)
    /// returns future of type [`Vec<f32>`] which is set once the function has completed
    async fn func_float32_async(
        &mut self,
        param_float32: &[f32],
    ) -> Result<Vec<f32>, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func_float32(param_float32))
    }

    fn func_float64(
        &mut self,
        _param_float: &[f64],
    ) -> Vec<f64> {
        Default::default()
    }
    /// Asynchronous version of [func_float64](SimpleArrayInterface::func_float64)
    /// returns future of type [`Vec<f64>`] which is set once the function has completed
    async fn func_float64_async(
        &mut self,
        param_float: &[f64],
    ) -> Result<Vec<f64>, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func_float64(param_float))
    }

    fn func_string(
        &mut self,
        _param_string: &[String],
    ) -> Vec<String> {
        Default::default()
    }
    /// Asynchronous version of [func_string](SimpleArrayInterface::func_string)
    /// returns future of type [`Vec<String>`] which is set once the function has completed
    async fn func_string_async(
        &mut self,
        param_string: &[String],
    ) -> Result<Vec<String>, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func_string(param_string))
    }

    /// Gets the value of the propBool property.
    fn prop_bool(&self) -> &Vec<bool> {
        &self.prop_bool
    }
    /// Sets the value of the propBool property.
    fn set_prop_bool(
        &mut self,
        prop_bool: &[bool],
    ) {
        if self.prop_bool == prop_bool.to_vec() {
            return;
        }

        self.prop_bool = prop_bool.to_vec();
        self._signal_handler.prop_bool_changed.emit(self.prop_bool.clone());
    }

    /// Gets the value of the propInt property.
    fn prop_int(&self) -> &Vec<i32> {
        &self.prop_int
    }
    /// Sets the value of the propInt property.
    fn set_prop_int(
        &mut self,
        prop_int: &[i32],
    ) {
        if self.prop_int == prop_int.to_vec() {
            return;
        }

        self.prop_int = prop_int.to_vec();
        self._signal_handler.prop_int_changed.emit(self.prop_int.clone());
    }

    /// Gets the value of the propInt32 property.
    fn prop_int32(&self) -> &Vec<i32> {
        &self.prop_int32
    }
    /// Sets the value of the propInt32 property.
    fn set_prop_int32(
        &mut self,
        prop_int32: &[i32],
    ) {
        if self.prop_int32 == prop_int32.to_vec() {
            return;
        }

        self.prop_int32 = prop_int32.to_vec();
        self._signal_handler.prop_int32_changed.emit(self.prop_int32.clone());
    }

    /// Gets the value of the propInt64 property.
    fn prop_int64(&self) -> &Vec<i64> {
        &self.prop_int64
    }
    /// Sets the value of the propInt64 property.
    fn set_prop_int64(
        &mut self,
        prop_int64: &[i64],
    ) {
        if self.prop_int64 == prop_int64.to_vec() {
            return;
        }

        self.prop_int64 = prop_int64.to_vec();
        self._signal_handler.prop_int64_changed.emit(self.prop_int64.clone());
    }

    /// Gets the value of the propFloat property.
    fn prop_float(&self) -> &Vec<f32> {
        &self.prop_float
    }
    /// Sets the value of the propFloat property.
    fn set_prop_float(
        &mut self,
        prop_float: &[f32],
    ) {
        if self.prop_float == prop_float.to_vec() {
            return;
        }

        self.prop_float = prop_float.to_vec();
        self._signal_handler.prop_float_changed.emit(self.prop_float.clone());
    }

    /// Gets the value of the propFloat32 property.
    fn prop_float32(&self) -> &Vec<f32> {
        &self.prop_float32
    }
    /// Sets the value of the propFloat32 property.
    fn set_prop_float32(
        &mut self,
        prop_float32: &[f32],
    ) {
        if self.prop_float32 == prop_float32.to_vec() {
            return;
        }

        self.prop_float32 = prop_float32.to_vec();
        self._signal_handler.prop_float32_changed.emit(self.prop_float32.clone());
    }

    /// Gets the value of the propFloat64 property.
    fn prop_float64(&self) -> &Vec<f64> {
        &self.prop_float64
    }
    /// Sets the value of the propFloat64 property.
    fn set_prop_float64(
        &mut self,
        prop_float64: &[f64],
    ) {
        if self.prop_float64 == prop_float64.to_vec() {
            return;
        }

        self.prop_float64 = prop_float64.to_vec();
        self._signal_handler.prop_float64_changed.emit(self.prop_float64.clone());
    }

    /// Gets the value of the propString property.
    fn prop_string(&self) -> &Vec<String> {
        &self.prop_string
    }
    /// Sets the value of the propString property.
    fn set_prop_string(
        &mut self,
        prop_string: &[String],
    ) {
        if self.prop_string == prop_string.to_vec() {
            return;
        }

        self.prop_string = prop_string.to_vec();
        self._signal_handler.prop_string_changed.emit(self.prop_string.clone());
    }

    fn _get_signal_handler(&mut self) -> &SimpleArrayInterfaceSignalHandler {
        &self._signal_handler
    }
}
