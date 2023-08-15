use std::future::Future;

pub trait VoidInterface {
    fn func_void(&mut self);
    /// Asynchronous version of `func_void`
    /// returns future of type () which is set once the function has completed
    fn func_void_async(&mut self) -> dyn Future<Output = ()>;
}
