// we have no simple way to detect whether a struct/enum is used
#[allow(unused_imports)]
use crate::api::data_structs::*;
use async_trait::async_trait;
use signals2::*;

#[derive(Clone, Default)]
pub struct SameEnum1InterfaceSignalHandler {
    pub prop1_changed: Signal<(Enum1Enum,)>,

    pub sig1: Signal<(Enum1Enum,)>,
}

#[async_trait]
pub trait SameEnum1InterfaceTrait {
    fn func1(
        &mut self,
        param1: Enum1Enum,
    ) -> Enum1Enum;
    /// Asynchronous version of [func1](SameEnum1InterfaceTrait::func1)
    /// returns future of type [`Enum1Enum`] which is set once the function has completed
    async fn func1_async(
        &mut self,
        param1: Enum1Enum,
    ) -> Result<Enum1Enum, ()>;

    /// Gets the value of the prop1 property.
    fn prop1(&self) -> Enum1Enum;
    /// Sets the value of the prop1 property.
    fn set_prop1(
        &mut self,
        prop1: Enum1Enum,
    );

    fn _get_signal_handler(&mut self) -> &SameEnum1InterfaceSignalHandler;
}
