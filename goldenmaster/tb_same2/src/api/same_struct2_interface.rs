// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use crate::api::data_structs::*;
use async_trait::async_trait;

#[async_trait]
pub trait SameStruct2InterfaceTrait {
    fn func1(
        &mut self,
        param1: &Struct1,
    ) -> Struct1;
    /// Asynchronous version of [func1](SameStruct2InterfaceTrait::func1)
    /// returns future of type [`Struct1`] which is set once the function has completed
    async fn func1_async(
        &mut self,
        param1: &Struct1,
    ) -> Result<Struct1, ()>;

    fn func2(
        &mut self,
        param1: &Struct1,
        param2: &Struct2,
    ) -> Struct1;
    /// Asynchronous version of [func2](SameStruct2InterfaceTrait::func2)
    /// returns future of type [`Struct1`] which is set once the function has completed
    async fn func2_async(
        &mut self,
        param1: &Struct1,
        param2: &Struct2,
    ) -> Result<Struct1, ()>;

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
