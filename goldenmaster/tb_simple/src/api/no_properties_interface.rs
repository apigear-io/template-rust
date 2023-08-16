use std::pin::Pin;
use std::future::Future;

pub trait NoPropertiesInterfaceTrait {
    fn func_void(&mut self);
    /// Asynchronous version of `func_void`
    /// returns future of type () which is set once the function has completed
    fn func_void_async(&mut self) -> Pin<Box<dyn Future<Output = Result<(), ()>> + Unpin>>;

    fn func_bool(
        &mut self,
        param_bool: bool,
    ) -> bool;
    /// Asynchronous version of `func_bool`
    /// returns future of type bool which is set once the function has completed
    fn func_bool_async(
        &mut self,
        param_bool: bool,
    ) -> Pin<Box<dyn Future<Output = Result<bool, ()>> + Unpin>>;
}
