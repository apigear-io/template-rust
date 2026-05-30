#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use crate::api::many_param_interface::ManyParamInterfacePublisher;
use crate::api::many_param_interface::ManyParamInterfaceTrait;
use tracing;

/// Trace decorator for ManyParamInterface.
/// Wraps any implementation and instruments all operations with tracing spans.
pub struct ManyParamInterfaceTraced<T: ManyParamInterfaceTrait> {
    inner: T,
}

impl<T: ManyParamInterfaceTrait> ManyParamInterfaceTraced<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: ManyParamInterfaceTrait> ManyParamInterfaceTrait for ManyParamInterfaceTraced<T> {
    fn func1(
        &self,
        param1: i32,
    ) -> ApiFuture<'_, Result<i32, ApiError>> {
        tracing::info!("ManyParamInterface::func1 called");
        self.inner.func1(param1)
    }

    fn func2(
        &self,
        param1: i32,
        param2: i32,
    ) -> ApiFuture<'_, Result<i32, ApiError>> {
        tracing::info!("ManyParamInterface::func2 called");
        self.inner.func2(param1, param2)
    }

    fn func3(
        &self,
        param1: i32,
        param2: i32,
        param3: i32,
    ) -> ApiFuture<'_, Result<i32, ApiError>> {
        tracing::info!("ManyParamInterface::func3 called");
        self.inner.func3(param1, param2, param3)
    }

    fn func4(
        &self,
        param1: i32,
        param2: i32,
        param3: i32,
        param4: i32,
    ) -> ApiFuture<'_, Result<i32, ApiError>> {
        tracing::info!("ManyParamInterface::func4 called");
        self.inner.func4(param1, param2, param3, param4)
    }

    fn prop1(&self) -> i32 {
        self.inner.prop1()
    }
    fn set_prop1(
        &self,
        prop1: i32,
    ) {
        tracing::info!("ManyParamInterface::set_prop1 called");
        self.inner.set_prop1(prop1);
    }

    fn prop2(&self) -> i32 {
        self.inner.prop2()
    }
    fn set_prop2(
        &self,
        prop2: i32,
    ) {
        tracing::info!("ManyParamInterface::set_prop2 called");
        self.inner.set_prop2(prop2);
    }

    fn prop3(&self) -> i32 {
        self.inner.prop3()
    }
    fn set_prop3(
        &self,
        prop3: i32,
    ) {
        tracing::info!("ManyParamInterface::set_prop3 called");
        self.inner.set_prop3(prop3);
    }

    fn prop4(&self) -> i32 {
        self.inner.prop4()
    }
    fn set_prop4(
        &self,
        prop4: i32,
    ) {
        tracing::info!("ManyParamInterface::set_prop4 called");
        self.inner.set_prop4(prop4);
    }

    fn publisher(&self) -> &ManyParamInterfacePublisher {
        self.inner.publisher()
    }
}
