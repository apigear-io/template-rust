#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use crate::api::nested_struct2_interface::NestedStruct2InterfacePublisher;
use crate::api::nested_struct2_interface::NestedStruct2InterfaceTrait;
use tracing;

/// Trace decorator for NestedStruct2Interface.
/// Wraps any implementation and instruments all operations with tracing spans.
pub struct NestedStruct2InterfaceTraced<T: NestedStruct2InterfaceTrait> {
    inner: T,
}

impl<T: NestedStruct2InterfaceTrait> NestedStruct2InterfaceTraced<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: NestedStruct2InterfaceTrait> NestedStruct2InterfaceTrait for NestedStruct2InterfaceTraced<T> {
    fn func1(
        &self,
        param1: &NestedStruct1,
    ) -> ApiFuture<'_, Result<NestedStruct1, ApiError>> {
        tracing::info!("NestedStruct2Interface::func1 called");
        self.inner.func1(param1)
    }

    fn func2(
        &self,
        param1: &NestedStruct1,
        param2: &NestedStruct2,
    ) -> ApiFuture<'_, Result<NestedStruct1, ApiError>> {
        tracing::info!("NestedStruct2Interface::func2 called");
        self.inner.func2(param1, param2)
    }

    fn prop1(&self) -> NestedStruct1 {
        self.inner.prop1()
    }
    fn set_prop1(
        &self,
        prop1: &NestedStruct1,
    ) {
        tracing::info!("NestedStruct2Interface::set_prop1 called");
        self.inner.set_prop1(prop1);
    }

    fn prop2(&self) -> NestedStruct2 {
        self.inner.prop2()
    }
    fn set_prop2(
        &self,
        prop2: &NestedStruct2,
    ) {
        tracing::info!("NestedStruct2Interface::set_prop2 called");
        self.inner.set_prop2(prop2);
    }

    fn publisher(&self) -> &NestedStruct2InterfacePublisher {
        self.inner.publisher()
    }
}
