#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use crate::api::enum_interface::EnumInterfacePublisher;
use crate::api::enum_interface::EnumInterfaceTrait;
use tracing;

/// Trace decorator for EnumInterface.
/// Wraps any implementation and instruments all operations with tracing spans.
pub struct EnumInterfaceTraced<T: EnumInterfaceTrait> {
    inner: T,
}

impl<T: EnumInterfaceTrait> EnumInterfaceTraced<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: EnumInterfaceTrait> EnumInterfaceTrait for EnumInterfaceTraced<T> {
    fn func0(
        &self,
        param0: Enum0Enum,
    ) -> ApiFuture<'_, Result<Enum0Enum, ApiError>> {
        tracing::info!("EnumInterface::func0 called");
        self.inner.func0(param0)
    }

    fn func1(
        &self,
        param1: Enum1Enum,
    ) -> ApiFuture<'_, Result<Enum1Enum, ApiError>> {
        tracing::info!("EnumInterface::func1 called");
        self.inner.func1(param1)
    }

    fn func2(
        &self,
        param2: Enum2Enum,
    ) -> ApiFuture<'_, Result<Enum2Enum, ApiError>> {
        tracing::info!("EnumInterface::func2 called");
        self.inner.func2(param2)
    }

    fn func3(
        &self,
        param3: Enum3Enum,
    ) -> ApiFuture<'_, Result<Enum3Enum, ApiError>> {
        tracing::info!("EnumInterface::func3 called");
        self.inner.func3(param3)
    }

    fn prop0(&self) -> Enum0Enum {
        self.inner.prop0()
    }
    fn set_prop0(
        &self,
        prop0: Enum0Enum,
    ) {
        tracing::info!("EnumInterface::set_prop0 called");
        self.inner.set_prop0(prop0);
    }

    fn prop1(&self) -> Enum1Enum {
        self.inner.prop1()
    }
    fn set_prop1(
        &self,
        prop1: Enum1Enum,
    ) {
        tracing::info!("EnumInterface::set_prop1 called");
        self.inner.set_prop1(prop1);
    }

    fn prop2(&self) -> Enum2Enum {
        self.inner.prop2()
    }
    fn set_prop2(
        &self,
        prop2: Enum2Enum,
    ) {
        tracing::info!("EnumInterface::set_prop2 called");
        self.inner.set_prop2(prop2);
    }

    fn prop3(&self) -> Enum3Enum {
        self.inner.prop3()
    }
    fn set_prop3(
        &self,
        prop3: Enum3Enum,
    ) {
        tracing::info!("EnumInterface::set_prop3 called");
        self.inner.set_prop3(prop3);
    }

    fn publisher(&self) -> &EnumInterfacePublisher {
        self.inner.publisher()
    }
}
