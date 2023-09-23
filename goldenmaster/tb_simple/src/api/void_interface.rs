use async_trait::async_trait;
use signals2::*;

#[derive(Clone, Default)]
pub struct VoidInterfaceSignalHandler {
    pub sig_void: Signal<()>,
}

#[async_trait]
pub trait VoidInterfaceTrait {
    fn func_void(&mut self);
    /// Asynchronous version of [func_void](VoidInterfaceTrait::func_void)
    /// returns future of type [`()`] which is set once the function has completed
    async fn func_void_async(&mut self) -> Result<(), ()>;

    fn _get_signal_handler(&mut self) -> &VoidInterfaceSignalHandler;
}
