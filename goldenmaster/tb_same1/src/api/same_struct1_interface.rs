// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use crate::api::data_structs::*;
use std::pin::Pin;
use std::future::Future;

pub trait SameStruct1InterfaceTrait {
    fn func1(
        &mut self,
        param1: &Struct1,
    ) -> Struct1;
    /// Asynchronous version of `func1`
    /// returns future of type Struct1 which is set once the function has completed
    fn func1_async(
        &mut self,
        param1: &Struct1,
    ) -> Pin<Box<dyn Future<Output = Result<Struct1, ()>> + Unpin>>;

    /// Gets the value of the prop1 property.
    fn prop1(&self) -> &Struct1;
    /// Sets the value of the prop1 property.
    fn set_prop1(
        &mut self,
        prop1: &Struct1,
    );
}
