use async_trait::async_trait;

#[async_trait]
pub trait VoidInterfaceTrait {
    fn func_void(&mut self);
    /// Asynchronous version of `func_void`
    /// returns future of type () which is set once the function has completed
    async fn func_void_async(&mut self) -> Result<(), ()>;
}
