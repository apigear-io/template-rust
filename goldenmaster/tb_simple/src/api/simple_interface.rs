use async_trait::async_trait;

#[async_trait]
pub trait SimpleInterfaceTrait {
    fn func_bool(
        &mut self,
        param_bool: bool,
    ) -> bool;
    /// Asynchronous version of [func_bool](SimpleInterfaceTrait::func_bool)
    /// returns future of type [`bool`] which is set once the function has completed
    async fn func_bool_async(
        &mut self,
        param_bool: bool,
    ) -> Result<bool, ()>;

    fn func_int(
        &mut self,
        param_int: i32,
    ) -> i32;
    /// Asynchronous version of [func_int](SimpleInterfaceTrait::func_int)
    /// returns future of type [`i32`] which is set once the function has completed
    async fn func_int_async(
        &mut self,
        param_int: i32,
    ) -> Result<i32, ()>;

    fn func_int32(
        &mut self,
        param_int32: i32,
    ) -> i32;
    /// Asynchronous version of [func_int32](SimpleInterfaceTrait::func_int32)
    /// returns future of type [`i32`] which is set once the function has completed
    async fn func_int32_async(
        &mut self,
        param_int32: i32,
    ) -> Result<i32, ()>;

    fn func_int64(
        &mut self,
        param_int64: i64,
    ) -> i64;
    /// Asynchronous version of [func_int64](SimpleInterfaceTrait::func_int64)
    /// returns future of type [`i64`] which is set once the function has completed
    async fn func_int64_async(
        &mut self,
        param_int64: i64,
    ) -> Result<i64, ()>;

    fn func_float(
        &mut self,
        param_float: f32,
    ) -> f32;
    /// Asynchronous version of [func_float](SimpleInterfaceTrait::func_float)
    /// returns future of type [`f32`] which is set once the function has completed
    async fn func_float_async(
        &mut self,
        param_float: f32,
    ) -> Result<f32, ()>;

    fn func_float32(
        &mut self,
        param_float32: f32,
    ) -> f32;
    /// Asynchronous version of [func_float32](SimpleInterfaceTrait::func_float32)
    /// returns future of type [`f32`] which is set once the function has completed
    async fn func_float32_async(
        &mut self,
        param_float32: f32,
    ) -> Result<f32, ()>;

    fn func_float64(
        &mut self,
        param_float: f64,
    ) -> f64;
    /// Asynchronous version of [func_float64](SimpleInterfaceTrait::func_float64)
    /// returns future of type [`f64`] which is set once the function has completed
    async fn func_float64_async(
        &mut self,
        param_float: f64,
    ) -> Result<f64, ()>;

    fn func_string(
        &mut self,
        param_string: &str,
    ) -> String;
    /// Asynchronous version of [func_string](SimpleInterfaceTrait::func_string)
    /// returns future of type [`String`] which is set once the function has completed
    async fn func_string_async(
        &mut self,
        param_string: &str,
    ) -> Result<String, ()>;

    /// Gets the value of the propBool property.
    fn prop_bool(&self) -> bool;
    /// Sets the value of the propBool property.
    fn set_prop_bool(
        &mut self,
        prop_bool: bool,
    );

    /// Gets the value of the propInt property.
    fn prop_int(&self) -> i32;
    /// Sets the value of the propInt property.
    fn set_prop_int(
        &mut self,
        prop_int: i32,
    );

    /// Gets the value of the propInt32 property.
    fn prop_int32(&self) -> i32;
    /// Sets the value of the propInt32 property.
    fn set_prop_int32(
        &mut self,
        prop_int32: i32,
    );

    /// Gets the value of the propInt64 property.
    fn prop_int64(&self) -> i64;
    /// Sets the value of the propInt64 property.
    fn set_prop_int64(
        &mut self,
        prop_int64: i64,
    );

    /// Gets the value of the propFloat property.
    fn prop_float(&self) -> f32;
    /// Sets the value of the propFloat property.
    fn set_prop_float(
        &mut self,
        prop_float: f32,
    );

    /// Gets the value of the propFloat32 property.
    fn prop_float32(&self) -> f32;
    /// Sets the value of the propFloat32 property.
    fn set_prop_float32(
        &mut self,
        prop_float32: f32,
    );

    /// Gets the value of the propFloat64 property.
    fn prop_float64(&self) -> f64;
    /// Sets the value of the propFloat64 property.
    fn set_prop_float64(
        &mut self,
        prop_float64: f64,
    );

    /// Gets the value of the propString property.
    fn prop_string(&self) -> &String;
    /// Sets the value of the propString property.
    fn set_prop_string(
        &mut self,
        prop_string: &str,
    );
}
