// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use crate::data_structs::*;
use std::future::Future;

pub trait SameEnum2Interface {
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

    fn func2(
        &mut self,
        param1: Enum1Enum,
        param2: Enum2Enum,
    ) -> Enum1Enum;
    /// Asynchronous version of `func2`
    /// returns future of type Enum1Enum which is set once the function has completed
    fn func2_async(
        &mut self,
        param1: Enum1Enum,
        param2: Enum2Enum,
    ) -> dyn Future<Output = Enum1Enum>;

    /// Gets the value of the prop1 property.
    fn prop1(&self) -> Enum1Enum;
    /// Sets the value of the prop1 property.
    fn set_prop1(
        &mut self,
        prop1: Enum1Enum,
    );

    /// Gets the value of the prop2 property.
    fn prop2(&self) -> Enum2Enum;
    /// Sets the value of the prop2 property.
    fn set_prop2(
        &mut self,
        prop2: Enum2Enum,
    );
}
