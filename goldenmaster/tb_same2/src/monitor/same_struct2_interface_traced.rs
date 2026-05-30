#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use crate::api::same_struct2_interface::SameStruct2InterfacePublisher;
use crate::api::same_struct2_interface::SameStruct2InterfaceTrait;
use tracing;

/// Trace decorator for SameStruct2Interface.
/// Wraps any implementation and instruments all operations with tracing spans.
pub struct SameStruct2InterfaceTraced<T: SameStruct2InterfaceTrait> {
    inner: T,
}

impl<T: SameStruct2InterfaceTrait> SameStruct2InterfaceTraced<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: SameStruct2InterfaceTrait> SameStruct2InterfaceTrait for SameStruct2InterfaceTraced<T> {
    fn func1(
        &self,
        param1: &Struct1,
    ) -> ApiFuture<'_, Result<Struct1, ApiError>> {
        tracing::info!("SameStruct2Interface::func1 called");
        self.inner.func1(param1)
    }

    fn func2(
        &self,
        param1: &Struct1,
        param2: &Struct2,
    ) -> ApiFuture<'_, Result<Struct1, ApiError>> {
        tracing::info!("SameStruct2Interface::func2 called");
        self.inner.func2(param1, param2)
    }

    fn prop1(&self) -> Struct2 {
        self.inner.prop1()
    }
    fn set_prop1(
        &self,
        prop1: &Struct2,
    ) {
        tracing::info!("SameStruct2Interface::set_prop1 called");
        self.inner.set_prop1(prop1);
    }

    fn prop2(&self) -> Struct2 {
        self.inner.prop2()
    }
    fn set_prop2(
        &self,
        prop2: &Struct2,
    ) {
        tracing::info!("SameStruct2Interface::set_prop2 called");
        self.inner.set_prop2(prop2);
    }

    fn publisher(&self) -> &SameStruct2InterfacePublisher {
        self.inner.publisher()
    }
}
