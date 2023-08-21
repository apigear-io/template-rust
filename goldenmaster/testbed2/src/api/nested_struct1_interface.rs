// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use crate::api::data_structs::*;
use async_trait::async_trait;

#[async_trait]
pub trait NestedStruct1InterfaceTrait {
    fn func1(
        &mut self,
        param1: &NestedStruct1,
    ) -> NestedStruct1;
    /// Asynchronous version of `func1`
    /// returns future of type NestedStruct1 which is set once the function has completed
    async fn func1_async(
        &mut self,
        param1: &NestedStruct1,
    ) -> Result<NestedStruct1, ()>;

    /// Gets the value of the prop1 property.
    fn prop1(&self) -> &NestedStruct1;
    /// Sets the value of the prop1 property.
    fn set_prop1(
        &mut self,
        prop1: &NestedStruct1,
    );
}
