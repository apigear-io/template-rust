use crate::api::simple_interface::SimpleInterfaceTrait;
use async_trait::async_trait;

#[derive(Default, Clone)]
pub struct SimpleInterface {
    prop_bool: bool,
    prop_int: i32,
    prop_int32: i32,
    prop_int64: i64,
    prop_float: f32,
    prop_float32: f32,
    prop_float64: f64,
    prop_string: String,
}

#[async_trait]
impl SimpleInterfaceTrait for SimpleInterface {
    fn func_bool(
        &mut self,
        _param_bool: bool,
    ) -> bool {
        Default::default()
    }
    /// Asynchronous version of [func_bool](SimpleInterface::func_bool)
    /// returns future of type [`bool`] which is set once the function has completed
    async fn func_bool_async(
        &mut self,
        param_bool: bool,
    ) -> Result<bool, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func_bool(param_bool))
    }

    fn func_int(
        &mut self,
        _param_int: i32,
    ) -> i32 {
        Default::default()
    }
    /// Asynchronous version of [func_int](SimpleInterface::func_int)
    /// returns future of type [`i32`] which is set once the function has completed
    async fn func_int_async(
        &mut self,
        param_int: i32,
    ) -> Result<i32, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func_int(param_int))
    }

    fn func_int32(
        &mut self,
        _param_int32: i32,
    ) -> i32 {
        Default::default()
    }
    /// Asynchronous version of [func_int32](SimpleInterface::func_int32)
    /// returns future of type [`i32`] which is set once the function has completed
    async fn func_int32_async(
        &mut self,
        param_int32: i32,
    ) -> Result<i32, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func_int32(param_int32))
    }

    fn func_int64(
        &mut self,
        _param_int64: i64,
    ) -> i64 {
        Default::default()
    }
    /// Asynchronous version of [func_int64](SimpleInterface::func_int64)
    /// returns future of type [`i64`] which is set once the function has completed
    async fn func_int64_async(
        &mut self,
        param_int64: i64,
    ) -> Result<i64, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func_int64(param_int64))
    }

    fn func_float(
        &mut self,
        _param_float: f32,
    ) -> f32 {
        Default::default()
    }
    /// Asynchronous version of [func_float](SimpleInterface::func_float)
    /// returns future of type [`f32`] which is set once the function has completed
    async fn func_float_async(
        &mut self,
        param_float: f32,
    ) -> Result<f32, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func_float(param_float))
    }

    fn func_float32(
        &mut self,
        _param_float32: f32,
    ) -> f32 {
        Default::default()
    }
    /// Asynchronous version of [func_float32](SimpleInterface::func_float32)
    /// returns future of type [`f32`] which is set once the function has completed
    async fn func_float32_async(
        &mut self,
        param_float32: f32,
    ) -> Result<f32, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func_float32(param_float32))
    }

    fn func_float64(
        &mut self,
        _param_float: f64,
    ) -> f64 {
        Default::default()
    }
    /// Asynchronous version of [func_float64](SimpleInterface::func_float64)
    /// returns future of type [`f64`] which is set once the function has completed
    async fn func_float64_async(
        &mut self,
        param_float: f64,
    ) -> Result<f64, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func_float64(param_float))
    }

    fn func_string(
        &mut self,
        _param_string: &str,
    ) -> String {
        Default::default()
    }
    /// Asynchronous version of [func_string](SimpleInterface::func_string)
    /// returns future of type [`String`] which is set once the function has completed
    async fn func_string_async(
        &mut self,
        param_string: &str,
    ) -> Result<String, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func_string(param_string))
    }

    /// Gets the value of the propBool property.
    fn prop_bool(&self) -> bool {
        self.prop_bool
    }
    /// Sets the value of the propBool property.
    fn set_prop_bool(
        &mut self,
        prop_bool: bool,
    ) {
        if self.prop_bool == prop_bool {
            return;
        }

        self.prop_bool = prop_bool;
    }

    /// Gets the value of the propInt property.
    fn prop_int(&self) -> i32 {
        self.prop_int
    }
    /// Sets the value of the propInt property.
    fn set_prop_int(
        &mut self,
        prop_int: i32,
    ) {
        if self.prop_int == prop_int {
            return;
        }

        self.prop_int = prop_int;
    }

    /// Gets the value of the propInt32 property.
    fn prop_int32(&self) -> i32 {
        self.prop_int32
    }
    /// Sets the value of the propInt32 property.
    fn set_prop_int32(
        &mut self,
        prop_int32: i32,
    ) {
        if self.prop_int32 == prop_int32 {
            return;
        }

        self.prop_int32 = prop_int32;
    }

    /// Gets the value of the propInt64 property.
    fn prop_int64(&self) -> i64 {
        self.prop_int64
    }
    /// Sets the value of the propInt64 property.
    fn set_prop_int64(
        &mut self,
        prop_int64: i64,
    ) {
        if self.prop_int64 == prop_int64 {
            return;
        }

        self.prop_int64 = prop_int64;
    }

    /// Gets the value of the propFloat property.
    fn prop_float(&self) -> f32 {
        self.prop_float
    }
    /// Sets the value of the propFloat property.
    fn set_prop_float(
        &mut self,
        prop_float: f32,
    ) {
        if self.prop_float == prop_float {
            return;
        }

        self.prop_float = prop_float;
    }

    /// Gets the value of the propFloat32 property.
    fn prop_float32(&self) -> f32 {
        self.prop_float32
    }
    /// Sets the value of the propFloat32 property.
    fn set_prop_float32(
        &mut self,
        prop_float32: f32,
    ) {
        if self.prop_float32 == prop_float32 {
            return;
        }

        self.prop_float32 = prop_float32;
    }

    /// Gets the value of the propFloat64 property.
    fn prop_float64(&self) -> f64 {
        self.prop_float64
    }
    /// Sets the value of the propFloat64 property.
    fn set_prop_float64(
        &mut self,
        prop_float64: f64,
    ) {
        if self.prop_float64 == prop_float64 {
            return;
        }

        self.prop_float64 = prop_float64;
    }

    /// Gets the value of the propString property.
    fn prop_string(&self) -> &String {
        &self.prop_string
    }
    /// Sets the value of the propString property.
    fn set_prop_string(
        &mut self,
        prop_string: &str,
    ) {
        if self.prop_string == prop_string {
            return;
        }

        self.prop_string = prop_string.to_string();
    }
}
