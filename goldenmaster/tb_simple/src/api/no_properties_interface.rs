use async_trait::async_trait;
use signals2::*;

#[derive(Clone, Default)]
pub struct NoPropertiesInterfaceSignalHandler {
    pub sig_void: Signal<()>,

    pub sig_bool: Signal<(bool,)>,
}

#[async_trait]
pub trait NoPropertiesInterfaceTrait {
    fn func_void(&mut self);
    /// Asynchronous version of [func_void](NoPropertiesInterfaceTrait::func_void)
    /// returns future of type [`()`] which is set once the function has completed
    async fn func_void_async(&mut self) -> Result<(), ()>;

    fn func_bool(
        &mut self,
        param_bool: bool,
    ) -> bool;
    /// Asynchronous version of [func_bool](NoPropertiesInterfaceTrait::func_bool)
    /// returns future of type [`bool`] which is set once the function has completed
    async fn func_bool_async(
        &mut self,
        param_bool: bool,
    ) -> Result<bool, ()>;

    fn _get_signal_handler(&mut self) -> &NoPropertiesInterfaceSignalHandler;
}
