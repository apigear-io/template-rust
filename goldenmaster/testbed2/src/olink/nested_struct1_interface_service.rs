#[allow(unused_imports)]
use crate::api::data_structs::*;
use crate::api::nested_struct1_interface::NestedStruct1InterfaceTrait;
use objectlink_core::traits::{IRemoteNode, ObjectSource};
use objectlink_core::types::Name;
use serde_json::{json, Value};
use std::sync::Arc;

/// OLink service adapter for NestedStruct1Interface.
/// Bridges a local implementation to the OLink protocol by implementing ObjectSource.
pub struct NestedStruct1InterfaceOlinkService {
    impl_: Arc<dyn NestedStruct1InterfaceTrait>,
}

impl NestedStruct1InterfaceOlinkService {
    pub fn new(impl_: Arc<dyn NestedStruct1InterfaceTrait>) -> Self {
        Self { impl_ }
    }
}

impl ObjectSource for NestedStruct1InterfaceOlinkService {
    fn olink_object_name(&self) -> &str {
        "testbed2.NestedStruct1Interface"
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
            "funcNoReturnValue" => {
                let param_0: NestedStruct1 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_no_return_value(&param_0))));
                match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                }
            }
            "funcNoParams" => {
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_no_params())));
                match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                }
            }
            "func1" => {
                let param_0: NestedStruct1 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func1(&param_0))));
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
        tracing::info!("NestedStruct1Interface linked");
    }

    fn olink_unlinked(
        &self,
        _object_id: &str,
    ) {
        tracing::info!("NestedStruct1Interface unlinked");
    }

    fn olink_collect_properties(&self) -> Value {
        json!({
            "prop1": self.impl_.prop1()
        })
    }
}
