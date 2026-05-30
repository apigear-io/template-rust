use crate::api::same_enum2_interface::SameEnum2InterfaceTrait;
#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use crate::api::same_enum2_interface::SameEnum2InterfacePublisher;
use parking_lot::RwLock;

pub struct SameEnum2Interface {
    prop1: RwLock<Enum1Enum>,
    prop2: RwLock<Enum2Enum>,
    publisher: SameEnum2InterfacePublisher,
}

impl Default for SameEnum2Interface {
    fn default() -> Self {
        Self { prop1: RwLock::new(Default::default()), prop2: RwLock::new(Default::default()), publisher: Default::default() }
    }
}

impl SameEnum2InterfaceTrait for SameEnum2Interface {
    fn func1(
        &self,
        _param1: Enum1Enum,
    ) -> ApiFuture<'_, Result<Enum1Enum, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn func2(
        &self,
        _param1: Enum1Enum,
        _param2: Enum2Enum,
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

    fn prop2(&self) -> Enum2Enum {
        *self.prop2.read()
    }
    fn set_prop2(
        &self,
        prop2: Enum2Enum,
    ) {
        let mut value = self.prop2.write();
        if *value == prop2 {
            return;
        }
        *value = prop2;
        let _ = self.publisher.prop2_changed.send(prop2);
    }

    fn publisher(&self) -> &SameEnum2InterfacePublisher {
        &self.publisher
    }
}
