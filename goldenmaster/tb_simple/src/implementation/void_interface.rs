use crate::api::void_interface::VoidInterfaceTrait;
use async_trait::async_trait;

#[derive(Default, Clone)]
pub struct VoidInterface {}

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
}
