#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use crate::api::nam_es::NamEsPublisher;
use crate::api::nam_es::NamEsTrait;
use tracing;

/// Trace decorator for NamEs.
/// Wraps any implementation and instruments all operations with tracing spans.
pub struct NamEsTraced<T: NamEsTrait> {
    inner: T,
}

impl<T: NamEsTrait> NamEsTraced<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: NamEsTrait> NamEsTrait for NamEsTraced<T> {
    fn some_function(
        &self,
        some_param: bool,
    ) -> ApiFuture<'_, Result<(), ApiError>> {
        tracing::info!("NamEs::some_function called");
        self.inner.some_function(some_param)
    }

    fn some_function2(
        &self,
        some_param: bool,
    ) -> ApiFuture<'_, Result<(), ApiError>> {
        tracing::info!("NamEs::some_function2 called");
        self.inner.some_function2(some_param)
    }

    fn switch(&self) -> bool {
        self.inner.switch()
    }
    fn set_switch(
        &self,
        switch: bool,
    ) {
        tracing::info!("NamEs::set_switch called");
        self.inner.set_switch(switch);
    }

    fn some_property(&self) -> i32 {
        self.inner.some_property()
    }
    fn set_some_property(
        &self,
        some_property: i32,
    ) {
        tracing::info!("NamEs::set_some_property called");
        self.inner.set_some_property(some_property);
    }

    fn some_poperty2(&self) -> i32 {
        self.inner.some_poperty2()
    }
    fn set_some_poperty2(
        &self,
        some_poperty2: i32,
    ) {
        tracing::info!("NamEs::set_some_poperty2 called");
        self.inner.set_some_poperty2(some_poperty2);
    }

    fn enum_property(&self) -> Enum_With_Under_scoresEnum {
        self.inner.enum_property()
    }
    fn set_enum_property(
        &self,
        enum_property: Enum_With_Under_scoresEnum,
    ) {
        tracing::info!("NamEs::set_enum_property called");
        self.inner.set_enum_property(enum_property);
    }

    fn publisher(&self) -> &NamEsPublisher {
        self.inner.publisher()
    }
}
