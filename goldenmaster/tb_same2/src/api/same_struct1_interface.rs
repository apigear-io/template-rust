#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use tokio::sync::{watch, broadcast};

pub struct SameStruct1InterfacePublisher {
    pub prop1_changed: watch::Sender<Struct1>,
    pub sig1: broadcast::Sender<(Struct1,)>,
}

impl Default for SameStruct1InterfacePublisher {
    fn default() -> Self {
        Self { prop1_changed: watch::channel(Default::default()).0, sig1: broadcast::Sender::new(16) }
    }
}

pub trait SameStruct1InterfaceTrait: Send + Sync {
    fn func1(
        &self,
        param1: &Struct1,
    ) -> ApiFuture<'_, Result<Struct1, ApiError>>;

    /// Gets the value of the prop1 property.
    fn prop1(&self) -> Struct1;
    /// Sets the value of the prop1 property.
    fn set_prop1(
        &self,
        prop1: &Struct1,
    );

    fn publisher(&self) -> &SameStruct1InterfacePublisher;
}
