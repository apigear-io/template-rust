use async_trait::async_trait;
use signals2::*;

#[derive(Clone, Default)]
pub struct NoSignalsInterfaceSignalHandler {
    pub prop_bool_changed: Signal<(bool,)>,

    pub prop_int_changed: Signal<(i32,)>,
}

#[async_trait]
pub trait NoSignalsInterfaceTrait {
    fn func_void(&mut self);
    /// Asynchronous version of [func_void](NoSignalsInterfaceTrait::func_void)
    /// returns future of type [`()`] which is set once the function has completed
    async fn func_void_async(&mut self) -> Result<(), ()>;

    fn func_bool(
        &mut self,
        param_bool: bool,
    ) -> bool;
    /// Asynchronous version of [func_bool](NoSignalsInterfaceTrait::func_bool)
    /// returns future of type [`bool`] which is set once the function has completed
    async fn func_bool_async(
        &mut self,
        param_bool: bool,
    ) -> Result<bool, ()>;

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

    fn _get_signal_handler(&mut self) -> &NoSignalsInterfaceSignalHandler;
}
