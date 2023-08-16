// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use crate::api::data_structs::*;
use std::pin::Pin;
use std::future::Future;

pub trait EnumInterfaceTrait {
    fn func0(
        &mut self,
        param0: Enum0Enum,
    ) -> Enum0Enum;
    /// Asynchronous version of `func0`
    /// returns future of type Enum0Enum which is set once the function has completed
    fn func0_async(
        &mut self,
        param0: Enum0Enum,
    ) -> Pin<Box<dyn Future<Output = Result<Enum0Enum, ()>> + Unpin>>;

    fn func1(
        &mut self,
        param1: Enum1Enum,
    ) -> Enum1Enum;
    /// Asynchronous version of `func1`
    /// returns future of type Enum1Enum which is set once the function has completed
    fn func1_async(
        &mut self,
        param1: Enum1Enum,
    ) -> Pin<Box<dyn Future<Output = Result<Enum1Enum, ()>> + Unpin>>;

    fn func2(
        &mut self,
        param2: Enum2Enum,
    ) -> Enum2Enum;
    /// Asynchronous version of `func2`
    /// returns future of type Enum2Enum which is set once the function has completed
    fn func2_async(
        &mut self,
        param2: Enum2Enum,
    ) -> Pin<Box<dyn Future<Output = Result<Enum2Enum, ()>> + Unpin>>;

    fn func3(
        &mut self,
        param3: Enum3Enum,
    ) -> Enum3Enum;
    /// Asynchronous version of `func3`
    /// returns future of type Enum3Enum which is set once the function has completed
    fn func3_async(
        &mut self,
        param3: Enum3Enum,
    ) -> Pin<Box<dyn Future<Output = Result<Enum3Enum, ()>> + Unpin>>;

    /// Gets the value of the prop0 property.
    fn prop0(&self) -> Enum0Enum;
    /// Sets the value of the prop0 property.
    fn set_prop0(
        &mut self,
        prop0: Enum0Enum,
    );

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

    /// Gets the value of the prop3 property.
    fn prop3(&self) -> Enum3Enum;
    /// Sets the value of the prop3 property.
    fn set_prop3(
        &mut self,
        prop3: Enum3Enum,
    );
}
