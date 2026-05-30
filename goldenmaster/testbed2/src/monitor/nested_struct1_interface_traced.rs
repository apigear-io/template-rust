#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use crate::api::nested_struct1_interface::NestedStruct1InterfacePublisher;
use crate::api::nested_struct1_interface::NestedStruct1InterfaceTrait;
use tracing;

/// Trace decorator for NestedStruct1Interface.
/// Wraps any implementation and instruments all operations with tracing spans.
pub struct NestedStruct1InterfaceTraced<T: NestedStruct1InterfaceTrait> {
    inner: T,
}

impl<T: NestedStruct1InterfaceTrait> NestedStruct1InterfaceTraced<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: NestedStruct1InterfaceTrait> NestedStruct1InterfaceTrait for NestedStruct1InterfaceTraced<T> {
    fn func_no_return_value(
        &self,
        param1: &NestedStruct1,
    ) -> ApiFuture<'_, Result<(), ApiError>> {
        tracing::info!("NestedStruct1Interface::func_no_return_value called");
        self.inner.func_no_return_value(param1)
    }

    fn func_no_params(&self) -> ApiFuture<'_, Result<NestedStruct1, ApiError>> {
        tracing::info!("NestedStruct1Interface::func_no_params called");
        self.inner.func_no_params()
    }

    fn func1(
        &self,
        param1: &NestedStruct1,
    ) -> ApiFuture<'_, Result<NestedStruct1, ApiError>> {
        tracing::info!("NestedStruct1Interface::func1 called");
        self.inner.func1(param1)
    }

    fn prop1(&self) -> NestedStruct1 {
        self.inner.prop1()
    }
    fn set_prop1(
        &self,
        prop1: &NestedStruct1,
    ) {
        tracing::info!("NestedStruct1Interface::set_prop1 called");
        self.inner.set_prop1(prop1);
    }

    fn publisher(&self) -> &NestedStruct1InterfacePublisher {
        self.inner.publisher()
    }
}
