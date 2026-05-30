#[allow(unused_imports)]
use crate::api::data_structs::*;
use crate::api::many_param_interface::ManyParamInterfaceTrait;
use objectlink_core::traits::{IRemoteNode, ObjectSource};
use objectlink_core::types::Name;
use serde_json::{json, Value};
use std::sync::Arc;

/// OLink service adapter for ManyParamInterface.
/// Bridges a local implementation to the OLink protocol by implementing ObjectSource.
pub struct ManyParamInterfaceOlinkService {
    impl_: Arc<dyn ManyParamInterfaceTrait>,
}

impl ManyParamInterfaceOlinkService {
    pub fn new(impl_: Arc<dyn ManyParamInterfaceTrait>) -> Self {
        Self { impl_ }
    }
}

impl ObjectSource for ManyParamInterfaceOlinkService {
    fn olink_object_name(&self) -> &str {
        "testbed2.ManyParamInterface"
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
                let param_0: i32 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func1(param_0))));
                match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                }
            }
            "func2" => {
                let param_0: i32 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let param_1: i32 = serde_json::from_value(arr.and_then(|a| a.get(1).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func2(param_0, param_1))));
                match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                }
            }
            "func3" => {
                let param_0: i32 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let param_1: i32 = serde_json::from_value(arr.and_then(|a| a.get(1).cloned()).unwrap_or_default()).unwrap_or_default();
                let param_2: i32 = serde_json::from_value(arr.and_then(|a| a.get(2).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func3(param_0, param_1, param_2))));
                match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                }
            }
            "func4" => {
                let param_0: i32 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let param_1: i32 = serde_json::from_value(arr.and_then(|a| a.get(1).cloned()).unwrap_or_default()).unwrap_or_default();
                let param_2: i32 = serde_json::from_value(arr.and_then(|a| a.get(2).cloned()).unwrap_or_default()).unwrap_or_default();
                let param_3: i32 = serde_json::from_value(arr.and_then(|a| a.get(3).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func4(param_0, param_1, param_2, param_3))));
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
                if let Ok(v) = serde_json::from_value::<i32>(value) {
                    self.impl_.set_prop1(v);
                }
            }
            "prop2" => {
                if let Ok(v) = serde_json::from_value::<i32>(value) {
                    self.impl_.set_prop2(v);
                }
            }
            "prop3" => {
                if let Ok(v) = serde_json::from_value::<i32>(value) {
                    self.impl_.set_prop3(v);
                }
            }
            "prop4" => {
                if let Ok(v) = serde_json::from_value::<i32>(value) {
                    self.impl_.set_prop4(v);
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
        tracing::info!("ManyParamInterface linked");
    }

    fn olink_unlinked(
        &self,
        _object_id: &str,
    ) {
        tracing::info!("ManyParamInterface unlinked");
    }

    fn olink_collect_properties(&self) -> Value {
        json!({
            "prop1": self.impl_.prop1(),
            "prop2": self.impl_.prop2(),
            "prop3": self.impl_.prop3(),
            "prop4": self.impl_.prop4()
        })
    }
}
