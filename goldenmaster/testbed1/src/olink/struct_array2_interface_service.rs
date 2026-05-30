#[allow(unused_imports)]
use crate::api::data_structs::*;
use crate::api::struct_array2_interface::StructArray2InterfaceTrait;
use objectlink_core::traits::{IRemoteNode, ObjectSource};
use objectlink_core::types::Name;
use serde_json::{json, Value};
use std::sync::Arc;

/// OLink service adapter for StructArray2Interface.
/// Bridges a local implementation to the OLink protocol by implementing ObjectSource.
pub struct StructArray2InterfaceOlinkService {
    impl_: Arc<dyn StructArray2InterfaceTrait>,
}

impl StructArray2InterfaceOlinkService {
    pub fn new(impl_: Arc<dyn StructArray2InterfaceTrait>) -> Self {
        Self { impl_ }
    }
}

impl ObjectSource for StructArray2InterfaceOlinkService {
    fn olink_object_name(&self) -> &str {
        "testbed1.StructArray2Interface"
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
                let param_0: StructBoolWithArray = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_bool(&param_0))));
                match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                }
            }
            "funcInt" => {
                let param_0: StructIntWithArray = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_int(&param_0))));
                match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                }
            }
            "funcFloat" => {
                let param_0: StructFloatWithArray = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_float(&param_0))));
                match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                }
            }
            "funcString" => {
                let param_0: StructStringWithArray = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_string(&param_0))));
                match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                }
            }
            "funcEnum" => {
                let param_0: StructEnumWithArray = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_enum(&param_0))));
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
                if let Ok(v) = serde_json::from_value::<StructBoolWithArray>(value) {
                    self.impl_.set_prop_bool(&v);
                }
            }
            "propInt" => {
                if let Ok(v) = serde_json::from_value::<StructIntWithArray>(value) {
                    self.impl_.set_prop_int(&v);
                }
            }
            "propFloat" => {
                if let Ok(v) = serde_json::from_value::<StructFloatWithArray>(value) {
                    self.impl_.set_prop_float(&v);
                }
            }
            "propString" => {
                if let Ok(v) = serde_json::from_value::<StructStringWithArray>(value) {
                    self.impl_.set_prop_string(&v);
                }
            }
            "propEnum" => {
                if let Ok(v) = serde_json::from_value::<StructEnumWithArray>(value) {
                    self.impl_.set_prop_enum(&v);
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
        tracing::info!("StructArray2Interface linked");
    }

    fn olink_unlinked(
        &self,
        _object_id: &str,
    ) {
        tracing::info!("StructArray2Interface unlinked");
    }

    fn olink_collect_properties(&self) -> Value {
        json!({
            "propBool": self.impl_.prop_bool(),
            "propInt": self.impl_.prop_int(),
            "propFloat": self.impl_.prop_float(),
            "propString": self.impl_.prop_string(),
            "propEnum": self.impl_.prop_enum()
        })
    }
}
