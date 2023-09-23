// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use crate::api::data_structs::*;
use async_trait::async_trait;
use signals2::*;

#[derive(Clone, Default)]
pub struct NestedStruct2InterfaceSignalHandler {
    pub prop1_changed: Signal<(NestedStruct1,)>,

    pub prop2_changed: Signal<(NestedStruct2,)>,

    pub sig1: Signal<(NestedStruct1,)>,

    pub sig2: Signal<(NestedStruct1, NestedStruct2)>,
}

#[async_trait]
pub trait NestedStruct2InterfaceTrait {
    fn func1(
        &mut self,
        param1: &NestedStruct1,
    ) -> NestedStruct1;
    /// Asynchronous version of [func1](NestedStruct2InterfaceTrait::func1)
    /// returns future of type [`NestedStruct1`] which is set once the function has completed
    async fn func1_async(
        &mut self,
        param1: &NestedStruct1,
    ) -> Result<NestedStruct1, ()>;

    fn func2(
        &mut self,
        param1: &NestedStruct1,
        param2: &NestedStruct2,
    ) -> NestedStruct1;
    /// Asynchronous version of [func2](NestedStruct2InterfaceTrait::func2)
    /// returns future of type [`NestedStruct1`] which is set once the function has completed
    async fn func2_async(
        &mut self,
        param1: &NestedStruct1,
        param2: &NestedStruct2,
    ) -> Result<NestedStruct1, ()>;

    /// Gets the value of the prop1 property.
    fn prop1(&self) -> &NestedStruct1;
    /// Sets the value of the prop1 property.
    fn set_prop1(
        &mut self,
        prop1: &NestedStruct1,
    );

    /// Gets the value of the prop2 property.
    fn prop2(&self) -> &NestedStruct2;
    /// Sets the value of the prop2 property.
    fn set_prop2(
        &mut self,
        prop2: &NestedStruct2,
    );

    fn _get_signal_handler(&mut self) -> &NestedStruct2InterfaceSignalHandler;
}
