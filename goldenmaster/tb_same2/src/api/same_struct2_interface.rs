#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use tokio::sync::{watch, broadcast};

pub struct SameStruct2InterfacePublisher {
    pub prop1_changed: watch::Sender<Struct2>,
    pub prop2_changed: watch::Sender<Struct2>,
    pub sig1: broadcast::Sender<(Struct1,)>,
    pub sig2: broadcast::Sender<(Struct1, Struct2)>,
}

impl Default for SameStruct2InterfacePublisher {
    fn default() -> Self {
        Self { prop1_changed: watch::channel(Default::default()).0, prop2_changed: watch::channel(Default::default()).0, sig1: broadcast::Sender::new(16), sig2: broadcast::Sender::new(16) }
    }
}

pub trait SameStruct2InterfaceTrait: Send + Sync {
    fn func1(
        &self,
        param1: &Struct1,
    ) -> ApiFuture<'_, Result<Struct1, ApiError>>;

    fn func2(
        &self,
        param1: &Struct1,
        param2: &Struct2,
    ) -> ApiFuture<'_, Result<Struct1, ApiError>>;

    /// Gets the value of the prop1 property.
    fn prop1(&self) -> Struct2;
    /// Sets the value of the prop1 property.
    fn set_prop1(
        &self,
        prop1: &Struct2,
    );

    /// Gets the value of the prop2 property.
    fn prop2(&self) -> Struct2;
    /// Sets the value of the prop2 property.
    fn set_prop2(
        &self,
        prop2: &Struct2,
    );

    fn publisher(&self) -> &SameStruct2InterfacePublisher;
}
