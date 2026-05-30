#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use crate::api::same_struct1_interface::SameStruct1InterfacePublisher;
use crate::api::same_struct1_interface::SameStruct1InterfaceTrait;
use tracing;

/// Trace decorator for SameStruct1Interface.
/// Wraps any implementation and instruments all operations with tracing spans.
pub struct SameStruct1InterfaceTraced<T: SameStruct1InterfaceTrait> {
    inner: T,
}

impl<T: SameStruct1InterfaceTrait> SameStruct1InterfaceTraced<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: SameStruct1InterfaceTrait> SameStruct1InterfaceTrait for SameStruct1InterfaceTraced<T> {
    fn func1(
        &self,
        param1: &Struct1,
    ) -> ApiFuture<'_, Result<Struct1, ApiError>> {
        tracing::info!("SameStruct1Interface::func1 called");
        self.inner.func1(param1)
    }

    fn prop1(&self) -> Struct1 {
        self.inner.prop1()
    }
    fn set_prop1(
        &self,
        prop1: &Struct1,
    ) {
        tracing::info!("SameStruct1Interface::set_prop1 called");
        self.inner.set_prop1(prop1);
    }

    fn publisher(&self) -> &SameStruct1InterfacePublisher {
        self.inner.publisher()
    }
}
