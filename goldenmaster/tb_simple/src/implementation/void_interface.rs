use crate::api::void_interface::VoidInterfaceTrait;
use async_trait::async_trait;
use crate::api::void_interface::VoidInterfaceSignalHandler;
#[allow(unused_imports)]
use signals2::*;

#[derive(Default, Clone)]
pub struct VoidInterface {
    _signal_handler: VoidInterfaceSignalHandler,
}

#[async_trait]
impl VoidInterfaceTrait for VoidInterface {
    fn func_void(&mut self) {
        Default::default()
    }
    /// Asynchronous version of [func_void](VoidInterface::func_void)
    /// returns future of type [`()`] which is set once the function has completed
    async fn func_void_async(&mut self) -> Result<(), ()> {
        #[allow(clippy::unit_arg)]
        Ok(self.func_void())
    }

    fn _get_signal_handler(&mut self) -> &VoidInterfaceSignalHandler {
        &self._signal_handler
    }
}
