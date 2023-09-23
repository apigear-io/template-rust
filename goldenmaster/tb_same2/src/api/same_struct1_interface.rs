// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use crate::api::data_structs::*;
use async_trait::async_trait;
use signals2::*;

#[derive(Clone, Default)]
pub struct SameStruct1InterfaceSignalHandler {
    pub prop1_changed: Signal<(Struct1,)>,

    pub sig1: Signal<(Struct1,)>,
}

#[async_trait]
pub trait SameStruct1InterfaceTrait {
    fn func1(
        &mut self,
        param1: &Struct1,
    ) -> Struct1;
    /// Asynchronous version of [func1](SameStruct1InterfaceTrait::func1)
    /// returns future of type [`Struct1`] which is set once the function has completed
    async fn func1_async(
        &mut self,
        param1: &Struct1,
    ) -> Result<Struct1, ()>;

    /// Gets the value of the prop1 property.
    fn prop1(&self) -> &Struct1;
    /// Sets the value of the prop1 property.
    fn set_prop1(
        &mut self,
        prop1: &Struct1,
    );

    fn _get_signal_handler(&mut self) -> &SameStruct1InterfaceSignalHandler;
}
