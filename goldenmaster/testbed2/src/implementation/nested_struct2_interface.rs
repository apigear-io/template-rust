use crate::api::nested_struct2_interface::NestedStruct2InterfaceTrait;
#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use crate::api::nested_struct2_interface::NestedStruct2InterfacePublisher;
use parking_lot::RwLock;

pub struct NestedStruct2Interface {
    prop1: RwLock<NestedStruct1>,
    prop2: RwLock<NestedStruct2>,
    publisher: NestedStruct2InterfacePublisher,
}

impl Default for NestedStruct2Interface {
    fn default() -> Self {
        Self { prop1: RwLock::new(Default::default()), prop2: RwLock::new(Default::default()), publisher: Default::default() }
    }
}

impl NestedStruct2InterfaceTrait for NestedStruct2Interface {
    fn func1(
        &self,
        _param1: &NestedStruct1,
    ) -> ApiFuture<'_, Result<NestedStruct1, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn func2(
        &self,
        _param1: &NestedStruct1,
        _param2: &NestedStruct2,
    ) -> ApiFuture<'_, Result<NestedStruct1, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn prop1(&self) -> NestedStruct1 {
        self.prop1.read().clone()
    }
    fn set_prop1(
        &self,
        prop1: &NestedStruct1,
    ) {
        let new_val = prop1.clone();
        let mut value = self.prop1.write();
        if *value == new_val {
            return;
        }
        *value = new_val.clone();
        let _ = self.publisher.prop1_changed.send(new_val);
    }

    fn prop2(&self) -> NestedStruct2 {
        self.prop2.read().clone()
    }
    fn set_prop2(
        &self,
        prop2: &NestedStruct2,
    ) {
        let new_val = prop2.clone();
        let mut value = self.prop2.write();
        if *value == new_val {
            return;
        }
        *value = new_val.clone();
        let _ = self.publisher.prop2_changed.send(new_val);
    }

    fn publisher(&self) -> &NestedStruct2InterfacePublisher {
        &self.publisher
    }
}
