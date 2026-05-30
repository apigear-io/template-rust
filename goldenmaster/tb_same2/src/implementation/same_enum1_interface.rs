use crate::api::same_enum1_interface::SameEnum1InterfaceTrait;
#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use crate::api::same_enum1_interface::SameEnum1InterfacePublisher;
use parking_lot::RwLock;

pub struct SameEnum1Interface {
    prop1: RwLock<Enum1Enum>,
    publisher: SameEnum1InterfacePublisher,
}

impl Default for SameEnum1Interface {
    fn default() -> Self {
        Self { prop1: RwLock::new(Default::default()), publisher: Default::default() }
    }
}

impl SameEnum1InterfaceTrait for SameEnum1Interface {
    fn func1(
        &self,
        _param1: Enum1Enum,
    ) -> ApiFuture<'_, Result<Enum1Enum, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn prop1(&self) -> Enum1Enum {
        *self.prop1.read()
    }
    fn set_prop1(
        &self,
        prop1: Enum1Enum,
    ) {
        let mut value = self.prop1.write();
        if *value == prop1 {
            return;
        }
        *value = prop1;
        let _ = self.publisher.prop1_changed.send(prop1);
    }

    fn publisher(&self) -> &SameEnum1InterfacePublisher {
        &self.publisher
    }
}
