use crate::api::nested_struct1_interface::NestedStruct1InterfaceTrait;
#[allow(unused_imports)]
use crate::api::data_structs::*;
use apigear::{ApiError, ApiFuture};
use crate::api::nested_struct1_interface::NestedStruct1InterfacePublisher;
use parking_lot::RwLock;

pub struct NestedStruct1Interface {
    prop1: RwLock<NestedStruct1>,
    publisher: NestedStruct1InterfacePublisher,
}

impl Default for NestedStruct1Interface {
    fn default() -> Self {
        Self { prop1: RwLock::new(Default::default()), publisher: Default::default() }
    }
}

impl NestedStruct1InterfaceTrait for NestedStruct1Interface {
    fn func_no_return_value(
        &self,
        _param1: &NestedStruct1,
    ) -> ApiFuture<'_, Result<(), ApiError>> {
        Box::pin(async move { Ok(()) })
    }

    fn func_no_params(&self) -> ApiFuture<'_, Result<NestedStruct1, ApiError>> {
        Box::pin(async move { Ok(Default::default()) })
    }

    fn func1(
        &self,
        _param1: &NestedStruct1,
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

    fn publisher(&self) -> &NestedStruct1InterfacePublisher {
        &self.publisher
    }
}
