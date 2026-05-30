#[allow(unused_imports)]
use crate::api::data_structs::*;
use crate::api::nested_struct2_interface::NestedStruct2InterfaceTrait;
use objectlink_core::traits::{IRemoteNode, ObjectSource};
use objectlink_core::types::Name;
use serde_json::{json, Value};
use std::sync::Arc;

/// OLink service adapter for NestedStruct2Interface.
/// Bridges a local implementation to the OLink protocol by implementing ObjectSource.
pub struct NestedStruct2InterfaceOlinkService {
    impl_: Arc<dyn NestedStruct2InterfaceTrait>,
}

impl NestedStruct2InterfaceOlinkService {
    pub fn new(impl_: Arc<dyn NestedStruct2InterfaceTrait>) -> Self {
        Self { impl_ }
    }
}

impl ObjectSource for NestedStruct2InterfaceOlinkService {
    fn olink_object_name(&self) -> &str {
        "testbed2.NestedStruct2Interface"
    }

    #[allow(clippy::get_first)]
    fn olink_invoke(
        &self,
        method_id: &str,
        args: Value,
    ) -> Value {
        let member = Name::member_name(method_id);
        #[allow(unused_variables)]
        let arr = args.as_array();
        match member {
            "func1" => {
                let param_0: NestedStruct1 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func1(&param_0))));
                match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                }
            }
            "func2" => {
                let param_0: NestedStruct1 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let param_1: NestedStruct2 = serde_json::from_value(arr.and_then(|a| a.get(1).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func2(&param_0, &param_1))));
                match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                }
            }
            _ => {
                tracing::warn!("Unknown method: {}", method_id);
                json!(null)
            }
        }
    }

    fn olink_set_property(
        &self,
        property_id: &str,
        value: Value,
    ) {
        let member = Name::member_name(property_id);
        match member {
            "prop1" => {
                if let Ok(v) = serde_json::from_value::<NestedStruct1>(value) {
                    self.impl_.set_prop1(&v);
                }
            }
            "prop2" => {
                if let Ok(v) = serde_json::from_value::<NestedStruct2>(value) {
                    self.impl_.set_prop2(&v);
                }
            }
            _ => {
                tracing::warn!("Unknown property: {}", property_id);
            }
        }
    }

    fn olink_linked(
        &self,
        _object_id: &str,
        _node: &dyn IRemoteNode,
    ) {
        tracing::info!("NestedStruct2Interface linked");
    }

    fn olink_unlinked(
        &self,
        _object_id: &str,
    ) {
        tracing::info!("NestedStruct2Interface unlinked");
    }

    fn olink_collect_properties(&self) -> Value {
        json!({
            "prop1": self.impl_.prop1(),
            "prop2": self.impl_.prop2()
        })
    }
}
