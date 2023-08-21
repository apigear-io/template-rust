// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use crate::api::data_structs::*;
use async_trait::async_trait;

#[async_trait]
pub trait ManyParamInterfaceTrait {
    fn func1(
        &mut self,
        param1: i32,
    ) -> i32;
    /// Asynchronous version of `func1`
    /// returns future of type i32 which is set once the function has completed
    async fn func1_async(
        &mut self,
        param1: i32,
    ) -> Result<i32, ()>;

    fn func2(
        &mut self,
        param1: i32,
        param2: i32,
    ) -> i32;
    /// Asynchronous version of `func2`
    /// returns future of type i32 which is set once the function has completed
    async fn func2_async(
        &mut self,
        param1: i32,
        param2: i32,
    ) -> Result<i32, ()>;

    fn func3(
        &mut self,
        param1: i32,
        param2: i32,
        param3: i32,
    ) -> i32;
    /// Asynchronous version of `func3`
    /// returns future of type i32 which is set once the function has completed
    async fn func3_async(
        &mut self,
        param1: i32,
        param2: i32,
        param3: i32,
    ) -> Result<i32, ()>;

    fn func4(
        &mut self,
        param1: i32,
        param2: i32,
        param3: i32,
        param4: i32,
    ) -> i32;
    /// Asynchronous version of `func4`
    /// returns future of type i32 which is set once the function has completed
    async fn func4_async(
        &mut self,
        param1: i32,
        param2: i32,
        param3: i32,
        param4: i32,
    ) -> Result<i32, ()>;

    /// Gets the value of the prop1 property.
    fn prop1(&self) -> i32;
    /// Sets the value of the prop1 property.
    fn set_prop1(
        &mut self,
        prop1: i32,
    );

    /// Gets the value of the prop2 property.
    fn prop2(&self) -> i32;
    /// Sets the value of the prop2 property.
    fn set_prop2(
        &mut self,
        prop2: i32,
    );

    /// Gets the value of the prop3 property.
    fn prop3(&self) -> i32;
    /// Sets the value of the prop3 property.
    fn set_prop3(
        &mut self,
        prop3: i32,
    );

    /// Gets the value of the prop4 property.
    fn prop4(&self) -> i32;
    /// Sets the value of the prop4 property.
    fn set_prop4(
        &mut self,
        prop4: i32,
    );
}
