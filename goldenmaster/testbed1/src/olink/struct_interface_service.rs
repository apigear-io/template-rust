#[allow(unused_imports)]
use crate::api::data_structs::*;
use crate::api::struct_interface::StructInterfaceTrait;
use objectlink_core::traits::{IRemoteNode, ObjectSource};
use objectlink_core::types::Name;
use serde_json::{json, Value};
use std::sync::Arc;

/// OLink service adapter for StructInterface.
/// Bridges a local implementation to the OLink protocol by implementing ObjectSource.
pub struct StructInterfaceOlinkService {
    impl_: Arc<dyn StructInterfaceTrait>,
}

impl StructInterfaceOlinkService {
    pub fn new(impl_: Arc<dyn StructInterfaceTrait>) -> Self {
        Self { impl_ }
    }
}

impl ObjectSource for StructInterfaceOlinkService {
    fn olink_object_name(&self) -> &str {
        "testbed1.StructInterface"
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
            "funcBool" => {
                let param_0: StructBool = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_bool(&param_0))));
                match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                }
            }
            "funcInt" => {
                let param_0: StructInt = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_int(&param_0))));
                match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                }
            }
            "funcFloat" => {
                let param_0: StructFloat = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_float(&param_0))));
                match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                }
            }
            "funcString" => {
                let param_0: StructString = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_string(&param_0))));
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
            "propBool" => {
                if let Ok(v) = serde_json::from_value::<StructBool>(value) {
                    self.impl_.set_prop_bool(&v);
                }
            }
            "propInt" => {
                if let Ok(v) = serde_json::from_value::<StructInt>(value) {
                    self.impl_.set_prop_int(&v);
                }
            }
            "propFloat" => {
                if let Ok(v) = serde_json::from_value::<StructFloat>(value) {
                    self.impl_.set_prop_float(&v);
                }
            }
            "propString" => {
                if let Ok(v) = serde_json::from_value::<StructString>(value) {
                    self.impl_.set_prop_string(&v);
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
        tracing::info!("StructInterface linked");
    }

    fn olink_unlinked(
        &self,
        _object_id: &str,
    ) {
        tracing::info!("StructInterface unlinked");
    }

    fn olink_collect_properties(&self) -> Value {
        json!({
            "propBool": self.impl_.prop_bool(),
            "propInt": self.impl_.prop_int(),
            "propFloat": self.impl_.prop_float(),
            "propString": self.impl_.prop_string()
        })
    }
}
