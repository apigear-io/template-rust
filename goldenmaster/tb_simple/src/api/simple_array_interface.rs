use async_trait::async_trait;
use signals2::*;

#[derive(Clone, Default)]
pub struct SimpleArrayInterfaceSignalHandler {
    pub prop_bool_changed: Signal<(Vec<bool>,)>,

    pub prop_int_changed: Signal<(Vec<i32>,)>,

    pub prop_int32_changed: Signal<(Vec<i32>,)>,

    pub prop_int64_changed: Signal<(Vec<i64>,)>,

    pub prop_float_changed: Signal<(Vec<f32>,)>,

    pub prop_float32_changed: Signal<(Vec<f32>,)>,

    pub prop_float64_changed: Signal<(Vec<f64>,)>,

    pub prop_string_changed: Signal<(Vec<String>,)>,

    pub sig_bool: Signal<(Vec<bool>,)>,

    pub sig_int: Signal<(Vec<i32>,)>,

    pub sig_int32: Signal<(Vec<i32>,)>,

    pub sig_int64: Signal<(Vec<i64>,)>,

    pub sig_float: Signal<(Vec<f32>,)>,

    pub sig_float32: Signal<(Vec<f32>,)>,

    pub sig_float64: Signal<(Vec<f64>,)>,

    pub sig_string: Signal<(Vec<String>,)>,
}

#[async_trait]
pub trait SimpleArrayInterfaceTrait {
    fn func_bool(
        &mut self,
        param_bool: &[bool],
    ) -> Vec<bool>;
    /// Asynchronous version of [func_bool](SimpleArrayInterfaceTrait::func_bool)
    /// returns future of type [`Vec<bool>`] which is set once the function has completed
    async fn func_bool_async(
        &mut self,
        param_bool: &[bool],
    ) -> Result<Vec<bool>, ()>;

    fn func_int(
        &mut self,
        param_int: &[i32],
    ) -> Vec<i32>;
    /// Asynchronous version of [func_int](SimpleArrayInterfaceTrait::func_int)
    /// returns future of type [`Vec<i32>`] which is set once the function has completed
    async fn func_int_async(
        &mut self,
        param_int: &[i32],
    ) -> Result<Vec<i32>, ()>;

    fn func_int32(
        &mut self,
        param_int32: &[i32],
    ) -> Vec<i32>;
    /// Asynchronous version of [func_int32](SimpleArrayInterfaceTrait::func_int32)
    /// returns future of type [`Vec<i32>`] which is set once the function has completed
    async fn func_int32_async(
        &mut self,
        param_int32: &[i32],
    ) -> Result<Vec<i32>, ()>;

    fn func_int64(
        &mut self,
        param_int64: &[i64],
    ) -> Vec<i64>;
    /// Asynchronous version of [func_int64](SimpleArrayInterfaceTrait::func_int64)
    /// returns future of type [`Vec<i64>`] which is set once the function has completed
    async fn func_int64_async(
        &mut self,
        param_int64: &[i64],
    ) -> Result<Vec<i64>, ()>;

    fn func_float(
        &mut self,
        param_float: &[f32],
    ) -> Vec<f32>;
    /// Asynchronous version of [func_float](SimpleArrayInterfaceTrait::func_float)
    /// returns future of type [`Vec<f32>`] which is set once the function has completed
    async fn func_float_async(
        &mut self,
        param_float: &[f32],
    ) -> Result<Vec<f32>, ()>;

    fn func_float32(
        &mut self,
        param_float32: &[f32],
    ) -> Vec<f32>;
    /// Asynchronous version of [func_float32](SimpleArrayInterfaceTrait::func_float32)
    /// returns future of type [`Vec<f32>`] which is set once the function has completed
    async fn func_float32_async(
        &mut self,
        param_float32: &[f32],
    ) -> Result<Vec<f32>, ()>;

    fn func_float64(
        &mut self,
        param_float: &[f64],
    ) -> Vec<f64>;
    /// Asynchronous version of [func_float64](SimpleArrayInterfaceTrait::func_float64)
    /// returns future of type [`Vec<f64>`] which is set once the function has completed
    async fn func_float64_async(
        &mut self,
        param_float: &[f64],
    ) -> Result<Vec<f64>, ()>;

    fn func_string(
        &mut self,
        param_string: &[String],
    ) -> Vec<String>;
    /// Asynchronous version of [func_string](SimpleArrayInterfaceTrait::func_string)
    /// returns future of type [`Vec<String>`] which is set once the function has completed
    async fn func_string_async(
        &mut self,
        param_string: &[String],
    ) -> Result<Vec<String>, ()>;

    /// Gets the value of the propBool property.
    fn prop_bool(&self) -> &Vec<bool>;
    /// Sets the value of the propBool property.
    fn set_prop_bool(
        &mut self,
        prop_bool: &[bool],
    );

    /// Gets the value of the propInt property.
    fn prop_int(&self) -> &Vec<i32>;
    /// Sets the value of the propInt property.
    fn set_prop_int(
        &mut self,
        prop_int: &[i32],
    );

    /// Gets the value of the propInt32 property.
    fn prop_int32(&self) -> &Vec<i32>;
    /// Sets the value of the propInt32 property.
    fn set_prop_int32(
        &mut self,
        prop_int32: &[i32],
    );

    /// Gets the value of the propInt64 property.
    fn prop_int64(&self) -> &Vec<i64>;
    /// Sets the value of the propInt64 property.
    fn set_prop_int64(
        &mut self,
        prop_int64: &[i64],
    );

    /// Gets the value of the propFloat property.
    fn prop_float(&self) -> &Vec<f32>;
    /// Sets the value of the propFloat property.
    fn set_prop_float(
        &mut self,
        prop_float: &[f32],
    );

    /// Gets the value of the propFloat32 property.
    fn prop_float32(&self) -> &Vec<f32>;
    /// Sets the value of the propFloat32 property.
    fn set_prop_float32(
        &mut self,
        prop_float32: &[f32],
    );

    /// Gets the value of the propFloat64 property.
    fn prop_float64(&self) -> &Vec<f64>;
    /// Sets the value of the propFloat64 property.
    fn set_prop_float64(
        &mut self,
        prop_float64: &[f64],
    );

    /// Gets the value of the propString property.
    fn prop_string(&self) -> &Vec<String>;
    /// Sets the value of the propString property.
    fn set_prop_string(
        &mut self,
        prop_string: &[String],
    );

    fn _get_signal_handler(&mut self) -> &SimpleArrayInterfaceSignalHandler;
}
