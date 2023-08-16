use crate::api::void_interface::VoidInterfaceTrait;
use std::pin::Pin;
use futures::{future, Future};

#[derive(Default, Clone)]
pub struct VoidInterface {}

impl VoidInterfaceTrait for VoidInterface {
    fn func_void(&mut self) {
        Default::default()
    }
    /// Asynchronous version of `func_void`
    /// returns future of type () which is set once the function has completed
    fn func_void_async(&mut self) -> Pin<Box<dyn Future<Output = Result<(), ()>> + Unpin>> {
        Box::pin({
            #[allow(clippy::unit_arg)]
            future::ok(Default::default())
        })
    }
}
