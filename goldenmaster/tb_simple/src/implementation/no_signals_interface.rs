use crate::api::no_signals_interface::NoSignalsInterfaceTrait;
use async_trait::async_trait;
use crate::api::no_signals_interface::NoSignalsInterfaceSignalHandler;
use signals2::*;

#[derive(Default, Clone)]
pub struct NoSignalsInterface {
    prop_bool: bool,
    prop_int: i32,
    _signal_handler: NoSignalsInterfaceSignalHandler,
}

#[async_trait]
impl NoSignalsInterfaceTrait for NoSignalsInterface {
    fn func_void(&mut self) {
        Default::default()
    }
    /// Asynchronous version of [func_void](NoSignalsInterface::func_void)
    /// returns future of type [`()`] which is set once the function has completed
    async fn func_void_async(&mut self) -> Result<(), ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func_void())
    }

    fn func_bool(
        &mut self,
        _param_bool: bool,
    ) -> bool {
        Default::default()
    }
    /// Asynchronous version of [func_bool](NoSignalsInterface::func_bool)
    /// returns future of type [`bool`] which is set once the function has completed
    async fn func_bool_async(
        &mut self,
        param_bool: bool,
    ) -> Result<bool, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func_bool(param_bool))
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
        self._signal_handler.prop_bool_changed.emit(self.prop_bool);
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
        self._signal_handler.prop_int_changed.emit(self.prop_int);
    }

    fn _get_signal_handler(&mut self) -> &NoSignalsInterfaceSignalHandler {
        &self._signal_handler
    }
}
