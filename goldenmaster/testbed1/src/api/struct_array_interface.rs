// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use crate::api::data_structs::*;
use async_trait::async_trait;
use signals2::*;

#[derive(Clone, Default)]
pub struct StructArrayInterfaceSignalHandler {
    pub prop_bool_changed: Signal<(Vec<StructBool>,)>,

    pub prop_int_changed: Signal<(Vec<StructInt>,)>,

    pub prop_float_changed: Signal<(Vec<StructFloat>,)>,

    pub prop_string_changed: Signal<(Vec<StructString>,)>,

    pub sig_bool: Signal<(Vec<StructBool>,)>,

    pub sig_int: Signal<(Vec<StructInt>,)>,

    pub sig_float: Signal<(Vec<StructFloat>,)>,

    pub sig_string: Signal<(Vec<StructString>,)>,
}

#[async_trait]
pub trait StructArrayInterfaceTrait {
    fn func_bool(
        &mut self,
        param_bool: &[StructBool],
    ) -> StructBool;
    /// Asynchronous version of [func_bool](StructArrayInterfaceTrait::func_bool)
    /// returns future of type [`StructBool`] which is set once the function has completed
    async fn func_bool_async(
        &mut self,
        param_bool: &[StructBool],
    ) -> Result<StructBool, ()>;

    fn func_int(
        &mut self,
        param_int: &[StructInt],
    ) -> StructBool;
    /// Asynchronous version of [func_int](StructArrayInterfaceTrait::func_int)
    /// returns future of type [`StructBool`] which is set once the function has completed
    async fn func_int_async(
        &mut self,
        param_int: &[StructInt],
    ) -> Result<StructBool, ()>;

    fn func_float(
        &mut self,
        param_float: &[StructFloat],
    ) -> StructBool;
    /// Asynchronous version of [func_float](StructArrayInterfaceTrait::func_float)
    /// returns future of type [`StructBool`] which is set once the function has completed
    async fn func_float_async(
        &mut self,
        param_float: &[StructFloat],
    ) -> Result<StructBool, ()>;

    fn func_string(
        &mut self,
        param_string: &[StructString],
    ) -> StructBool;
    /// Asynchronous version of [func_string](StructArrayInterfaceTrait::func_string)
    /// returns future of type [`StructBool`] which is set once the function has completed
    async fn func_string_async(
        &mut self,
        param_string: &[StructString],
    ) -> Result<StructBool, ()>;

    /// Gets the value of the propBool property.
    fn prop_bool(&self) -> &Vec<StructBool>;
    /// Sets the value of the propBool property.
    fn set_prop_bool(
        &mut self,
        prop_bool: &[StructBool],
    );

    /// Gets the value of the propInt property.
    fn prop_int(&self) -> &Vec<StructInt>;
    /// Sets the value of the propInt property.
    fn set_prop_int(
        &mut self,
        prop_int: &[StructInt],
    );

    /// Gets the value of the propFloat property.
    fn prop_float(&self) -> &Vec<StructFloat>;
    /// Sets the value of the propFloat property.
    fn set_prop_float(
        &mut self,
        prop_float: &[StructFloat],
    );

    /// Gets the value of the propString property.
    fn prop_string(&self) -> &Vec<StructString>;
    /// Sets the value of the propString property.
    fn set_prop_string(
        &mut self,
        prop_string: &[StructString],
    );

    fn _get_signal_handler(&mut self) -> &StructArrayInterfaceSignalHandler;
}
