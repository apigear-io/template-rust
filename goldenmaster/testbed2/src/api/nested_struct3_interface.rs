// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use crate::api::data_structs::*;
use async_trait::async_trait;
use signals2::*;

#[derive(Clone, Default)]
pub struct NestedStruct3InterfaceSignalHandler {
    pub prop1_changed: Signal<(NestedStruct1,)>,

    pub prop2_changed: Signal<(NestedStruct2,)>,

    pub prop3_changed: Signal<(NestedStruct3,)>,

    pub sig1: Signal<(NestedStruct1,)>,

    pub sig2: Signal<(NestedStruct1, NestedStruct2)>,

    pub sig3: Signal<(NestedStruct1, NestedStruct2, NestedStruct3)>,
}

#[async_trait]
pub trait NestedStruct3InterfaceTrait {
    fn func1(
        &mut self,
        param1: &NestedStruct1,
    ) -> NestedStruct1;
    /// Asynchronous version of [func1](NestedStruct3InterfaceTrait::func1)
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
    /// Asynchronous version of [func2](NestedStruct3InterfaceTrait::func2)
    /// returns future of type [`NestedStruct1`] which is set once the function has completed
    async fn func2_async(
        &mut self,
        param1: &NestedStruct1,
        param2: &NestedStruct2,
    ) -> Result<NestedStruct1, ()>;

    fn func3(
        &mut self,
        param1: &NestedStruct1,
        param2: &NestedStruct2,
        param3: &NestedStruct3,
    ) -> NestedStruct1;
    /// Asynchronous version of [func3](NestedStruct3InterfaceTrait::func3)
    /// returns future of type [`NestedStruct1`] which is set once the function has completed
    async fn func3_async(
        &mut self,
        param1: &NestedStruct1,
        param2: &NestedStruct2,
        param3: &NestedStruct3,
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

    /// Gets the value of the prop3 property.
    fn prop3(&self) -> &NestedStruct3;
    /// Sets the value of the prop3 property.
    fn set_prop3(
        &mut self,
        prop3: &NestedStruct3,
    );

    fn _get_signal_handler(&mut self) -> &NestedStruct3InterfaceSignalHandler;
}
