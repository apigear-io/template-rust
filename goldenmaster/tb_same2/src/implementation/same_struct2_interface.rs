use crate::api::same_struct2_interface::SameStruct2InterfaceTrait;
#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use crate::api::same_struct2_interface::SameStruct2InterfacePublisher;
use parking_lot::RwLock;

pub struct SameStruct2Interface {
    prop1: RwLock<Struct2>,
    prop2: RwLock<Struct2>,
    publisher: SameStruct2InterfacePublisher,
}

impl Default for SameStruct2Interface {
    fn default() -> Self {
        Self { prop1: RwLock::new(Default::default()), prop2: RwLock::new(Default::default()), publisher: Default::default() }
    }
}

impl SameStruct2InterfaceTrait for SameStruct2Interface {
    fn func1(
        &self,
        _param1: &Struct1,
    ) -> ApiFuture<'_, Result<Struct1, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn func2(
        &self,
        _param1: &Struct1,
        _param2: &Struct2,
    ) -> ApiFuture<'_, Result<Struct1, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn prop1(&self) -> Struct2 {
        self.prop1.read().clone()
    }
    fn set_prop1(
        &self,
        prop1: &Struct2,
    ) {
        let new_val = prop1.clone();
        let mut value = self.prop1.write();
        if *value == new_val {
            return;
        }
        *value = new_val.clone();
        let _ = self.publisher.prop1_changed.send(new_val);
    }

    fn prop2(&self) -> Struct2 {
        self.prop2.read().clone()
    }
    fn set_prop2(
        &self,
        prop2: &Struct2,
    ) {
        let new_val = prop2.clone();
        let mut value = self.prop2.write();
        if *value == new_val {
            return;
        }
        *value = new_val.clone();
        let _ = self.publisher.prop2_changed.send(new_val);
    }

    fn publisher(&self) -> &SameStruct2InterfacePublisher {
        &self.publisher
    }
}
