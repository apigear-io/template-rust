#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use crate::api::same_enum2_interface::SameEnum2InterfacePublisher;
use crate::api::same_enum2_interface::SameEnum2InterfaceTrait;
use tracing;

/// Trace decorator for SameEnum2Interface.
/// Wraps any implementation and instruments all operations with tracing spans.
pub struct SameEnum2InterfaceTraced<T: SameEnum2InterfaceTrait> {
    inner: T,
}

impl<T: SameEnum2InterfaceTrait> SameEnum2InterfaceTraced<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: SameEnum2InterfaceTrait> SameEnum2InterfaceTrait for SameEnum2InterfaceTraced<T> {
    fn func1(
        &self,
        param1: Enum1Enum,
    ) -> ApiFuture<'_, Result<Enum1Enum, ApiError>> {
        tracing::info!("SameEnum2Interface::func1 called");
        self.inner.func1(param1)
    }

    fn func2(
        &self,
        param1: Enum1Enum,
        param2: Enum2Enum,
    ) -> ApiFuture<'_, Result<Enum1Enum, ApiError>> {
        tracing::info!("SameEnum2Interface::func2 called");
        self.inner.func2(param1, param2)
    }

    fn prop1(&self) -> Enum1Enum {
        self.inner.prop1()
    }
    fn set_prop1(
        &self,
        prop1: Enum1Enum,
    ) {
        tracing::info!("SameEnum2Interface::set_prop1 called");
        self.inner.set_prop1(prop1);
    }

    fn prop2(&self) -> Enum2Enum {
        self.inner.prop2()
    }
    fn set_prop2(
        &self,
        prop2: Enum2Enum,
    ) {
        tracing::info!("SameEnum2Interface::set_prop2 called");
        self.inner.set_prop2(prop2);
    }

    fn publisher(&self) -> &SameEnum2InterfacePublisher {
        self.inner.publisher()
    }
}
