use crate::api::no_properties_interface::NoPropertiesInterfaceTrait;
use async_trait::async_trait;
use crate::api::no_properties_interface::NoPropertiesInterfaceSignalHandler;
#[allow(unused_imports)]
use signals2::*;

#[derive(Default, Clone)]
pub struct NoPropertiesInterface {
    _signal_handler: NoPropertiesInterfaceSignalHandler,
}

#[async_trait]
impl NoPropertiesInterfaceTrait for NoPropertiesInterface {
    fn func_void(&mut self) {
        Default::default()
    }
    /// Asynchronous version of [func_void](NoPropertiesInterface::func_void)
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
    /// Asynchronous version of [func_bool](NoPropertiesInterface::func_bool)
    /// returns future of type [`bool`] which is set once the function has completed
    async fn func_bool_async(
        &mut self,
        param_bool: bool,
    ) -> Result<bool, ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func_bool(param_bool))
    }

    fn _get_signal_handler(&mut self) -> &NoPropertiesInterfaceSignalHandler {
        &self._signal_handler
    }
}
