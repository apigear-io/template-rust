// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use crate::data_structs::*;
use std::future::Future;

pub trait SameEnum1Interface {
    fn func1(
        &mut self,
        param1: Enum1Enum,
    ) -> Enum1Enum;
    /// Asynchronous version of `func1`
    /// returns future of type Enum1Enum which is set once the function has completed
    fn func1_async(
        &mut self,
        param1: Enum1Enum,
    ) -> dyn Future<Output = Enum1Enum>;

    /// Gets the value of the prop1 property.
    fn prop1(&self) -> Enum1Enum;
    /// Sets the value of the prop1 property.
    fn set_prop1(
        &mut self,
        prop1: Enum1Enum,
    );
}
