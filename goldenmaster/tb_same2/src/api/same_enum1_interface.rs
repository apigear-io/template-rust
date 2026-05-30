#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use tokio::sync::{watch, broadcast};

pub struct SameEnum1InterfacePublisher {
    pub prop1_changed: watch::Sender<Enum1Enum>,
    pub sig1: broadcast::Sender<(Enum1Enum,)>,
}

impl Default for SameEnum1InterfacePublisher {
    fn default() -> Self {
        Self { prop1_changed: watch::channel(Default::default()).0, sig1: broadcast::Sender::new(16) }
    }
}

pub trait SameEnum1InterfaceTrait: Send + Sync {
    fn func1(
        &self,
        param1: Enum1Enum,
    ) -> ApiFuture<'_, Result<Enum1Enum, ApiError>>;

    /// Gets the value of the prop1 property.
    fn prop1(&self) -> Enum1Enum;
    /// Sets the value of the prop1 property.
    fn set_prop1(
        &self,
        prop1: Enum1Enum,
    );

    fn publisher(&self) -> &SameEnum1InterfacePublisher;
}
