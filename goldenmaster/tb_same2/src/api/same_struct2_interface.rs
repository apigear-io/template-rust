// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use crate::data_structs::*;
use std::future::Future;

pub trait SameStruct2Interface {
    fn func1(
        &mut self,
        param1: &Struct1,
    ) -> Struct1;
    /// Asynchronous version of `func1`
    /// returns future of type Struct1 which is set once the function has completed
    fn func1_async(
        &mut self,
        param1: &Struct1,
    ) -> dyn Future<Output = Struct1>;

    fn func2(
        &mut self,
        param1: &Struct1,
        param2: &Struct2,
    ) -> Struct1;
    /// Asynchronous version of `func2`
    /// returns future of type Struct1 which is set once the function has completed
    fn func2_async(
        &mut self,
        param1: &Struct1,
        param2: &Struct2,
    ) -> dyn Future<Output = Struct1>;

    /// Gets the value of the prop1 property.
    fn prop1(&self) -> &Struct2;
    /// Sets the value of the prop1 property.
    fn set_prop1(
        &mut self,
        prop1: &Struct2,
    );

    /// Gets the value of the prop2 property.
    fn prop2(&self) -> &Struct2;
    /// Sets the value of the prop2 property.
    fn set_prop2(
        &mut self,
        prop2: &Struct2,
    );
}
