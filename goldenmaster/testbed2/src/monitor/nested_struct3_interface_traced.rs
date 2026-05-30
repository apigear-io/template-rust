#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use crate::api::nested_struct3_interface::NestedStruct3InterfacePublisher;
use crate::api::nested_struct3_interface::NestedStruct3InterfaceTrait;
use tracing;

/// Trace decorator for NestedStruct3Interface.
/// Wraps any implementation and instruments all operations with tracing spans.
pub struct NestedStruct3InterfaceTraced<T: NestedStruct3InterfaceTrait> {
    inner: T,
}

impl<T: NestedStruct3InterfaceTrait> NestedStruct3InterfaceTraced<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: NestedStruct3InterfaceTrait> NestedStruct3InterfaceTrait for NestedStruct3InterfaceTraced<T> {
    fn func1(
        &self,
        param1: &NestedStruct1,
    ) -> ApiFuture<'_, Result<NestedStruct1, ApiError>> {
        tracing::info!("NestedStruct3Interface::func1 called");
        self.inner.func1(param1)
    }

    fn func2(
        &self,
        param1: &NestedStruct1,
        param2: &NestedStruct2,
    ) -> ApiFuture<'_, Result<NestedStruct1, ApiError>> {
        tracing::info!("NestedStruct3Interface::func2 called");
        self.inner.func2(param1, param2)
    }

    fn func3(
        &self,
        param1: &NestedStruct1,
        param2: &NestedStruct2,
        param3: &NestedStruct3,
    ) -> ApiFuture<'_, Result<NestedStruct1, ApiError>> {
        tracing::info!("NestedStruct3Interface::func3 called");
        self.inner.func3(param1, param2, param3)
    }

    fn prop1(&self) -> NestedStruct1 {
        self.inner.prop1()
    }
    fn set_prop1(
        &self,
        prop1: &NestedStruct1,
    ) {
        tracing::info!("NestedStruct3Interface::set_prop1 called");
        self.inner.set_prop1(prop1);
    }

    fn prop2(&self) -> NestedStruct2 {
        self.inner.prop2()
    }
    fn set_prop2(
        &self,
        prop2: &NestedStruct2,
    ) {
        tracing::info!("NestedStruct3Interface::set_prop2 called");
        self.inner.set_prop2(prop2);
    }

    fn prop3(&self) -> NestedStruct3 {
        self.inner.prop3()
    }
    fn set_prop3(
        &self,
        prop3: &NestedStruct3,
    ) {
        tracing::info!("NestedStruct3Interface::set_prop3 called");
        self.inner.set_prop3(prop3);
    }

    fn publisher(&self) -> &NestedStruct3InterfacePublisher {
        self.inner.publisher()
    }
}
