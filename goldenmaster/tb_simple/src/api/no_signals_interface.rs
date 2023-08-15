use std::future::Future;

pub trait NoSignalsInterface {
    fn func_void(&mut self);
    /// Asynchronous version of `func_void`
    /// returns future of type () which is set once the function has completed
    fn func_void_async(&mut self) -> dyn Future<Output = ()>;

    fn func_bool(
        &mut self,
        param_bool: bool,
    ) -> bool;
    /// Asynchronous version of `func_bool`
    /// returns future of type bool which is set once the function has completed
    fn func_bool_async(
        &mut self,
        param_bool: bool,
    ) -> dyn Future<Output = bool>;

    /// Gets the value of the propBool property.
    fn prop_bool(&self) -> bool;
    /// Sets the value of the propBool property.
    fn set_prop_bool(
        &mut self,
        prop_bool: bool,
    );

    /// Gets the value of the propInt property.
    fn prop_int(&self) -> i32;
    /// Sets the value of the propInt property.
    fn set_prop_int(
        &mut self,
        prop_int: i32,
    );
}
