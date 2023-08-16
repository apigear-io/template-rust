use std::pin::Pin;
use std::future::Future;

pub trait VoidInterfaceTrait {
    fn func_void(&mut self);
    /// Asynchronous version of `func_void`
    /// returns future of type () which is set once the function has completed
    fn func_void_async(&mut self) -> Pin<Box<dyn Future<Output = Result<(), ()>> + Unpin>>;
}
