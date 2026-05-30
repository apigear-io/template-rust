#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use crate::api::same_enum1_interface::SameEnum1InterfacePublisher;
use crate::api::same_enum1_interface::SameEnum1InterfaceTrait;
use tracing;

/// Trace decorator for SameEnum1Interface.
/// Wraps any implementation and instruments all operations with tracing spans.
pub struct SameEnum1InterfaceTraced<T: SameEnum1InterfaceTrait> {
    inner: T,
}

impl<T: SameEnum1InterfaceTrait> SameEnum1InterfaceTraced<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: SameEnum1InterfaceTrait> SameEnum1InterfaceTrait for SameEnum1InterfaceTraced<T> {
    fn func1(
        &self,
        param1: Enum1Enum,
    ) -> ApiFuture<'_, Result<Enum1Enum, ApiError>> {
        tracing::info!("SameEnum1Interface::func1 called");
        self.inner.func1(param1)
    }

    fn prop1(&self) -> Enum1Enum {
        self.inner.prop1()
    }
    fn set_prop1(
        &self,
        prop1: Enum1Enum,
    ) {
        tracing::info!("SameEnum1Interface::set_prop1 called");
        self.inner.set_prop1(prop1);
    }

    fn publisher(&self) -> &SameEnum1InterfacePublisher {
        self.inner.publisher()
    }
}
