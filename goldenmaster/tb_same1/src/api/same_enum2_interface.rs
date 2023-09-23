// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use crate::api::data_structs::*;
use async_trait::async_trait;
use signals2::*;

#[derive(Clone, Default)]
pub struct SameEnum2InterfaceSignalHandler {
    pub prop1_changed: Signal<(Enum1Enum,)>,

    pub prop2_changed: Signal<(Enum2Enum,)>,

    pub sig1: Signal<(Enum1Enum,)>,

    pub sig2: Signal<(Enum1Enum, Enum2Enum)>,
}

#[async_trait]
pub trait SameEnum2InterfaceTrait {
    fn func1(
        &mut self,
        param1: Enum1Enum,
    ) -> Enum1Enum;
    /// Asynchronous version of [func1](SameEnum2InterfaceTrait::func1)
    /// returns future of type [`Enum1Enum`] which is set once the function has completed
    async fn func1_async(
        &mut self,
        param1: Enum1Enum,
    ) -> Result<Enum1Enum, ()>;

    fn func2(
        &mut self,
        param1: Enum1Enum,
        param2: Enum2Enum,
    ) -> Enum1Enum;
    /// Asynchronous version of [func2](SameEnum2InterfaceTrait::func2)
    /// returns future of type [`Enum1Enum`] which is set once the function has completed
    async fn func2_async(
        &mut self,
        param1: Enum1Enum,
        param2: Enum2Enum,
    ) -> Result<Enum1Enum, ()>;

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

    fn _get_signal_handler(&mut self) -> &SameEnum2InterfaceSignalHandler;
}
