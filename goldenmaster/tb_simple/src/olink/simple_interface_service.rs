use crate::api::simple_interface::SimpleInterfaceTrait;
use objectlink_core::traits::{IRemoteNode, ObjectSource};
use objectlink_core::types::Name;
use serde_json::{json, Value};
use std::sync::Arc;

/// OLink service adapter for SimpleInterface.
/// Bridges a local implementation to the OLink protocol by implementing ObjectSource.
pub struct SimpleInterfaceOlinkService {
    impl_: Arc<dyn SimpleInterfaceTrait>,
}

impl SimpleInterfaceOlinkService {
    pub fn new(impl_: Arc<dyn SimpleInterfaceTrait>) -> Self {
        Self { impl_ }
    }
}

impl ObjectSource for SimpleInterfaceOlinkService {
    fn olink_object_name(&self) -> &str {
        "tb.simple.SimpleInterface"
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
                let param_0: bool = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_no_return_value(param_0))));
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
            "funcBool" => {
                let param_0: bool = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_bool(param_0))));
                match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                }
            }
            "funcInt" => {
                let param_0: i32 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_int(param_0))));
                match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                }
            }
            "funcInt32" => {
                let param_0: i32 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_int32(param_0))));
                match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                }
            }
            "funcInt64" => {
                let param_0: i64 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_int64(param_0))));
                match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                }
            }
            "funcFloat" => {
                let param_0: f32 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_float(param_0))));
                match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                }
            }
            "funcFloat32" => {
                let param_0: f32 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_float32(param_0))));
                match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                }
            }
            "funcFloat64" => {
                let param_0: f64 = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_float64(param_0))));
                match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                }
            }
            "funcString" => {
                let param_0: String = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_string(param_0.as_str()))));
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
                if let Ok(v) = serde_json::from_value::<bool>(value) {
                    self.impl_.set_prop_bool(v);
                }
            }
            "propInt" => {
                if let Ok(v) = serde_json::from_value::<i32>(value) {
                    self.impl_.set_prop_int(v);
                }
            }
            "propInt32" => {
                if let Ok(v) = serde_json::from_value::<i32>(value) {
                    self.impl_.set_prop_int32(v);
                }
            }
            "propInt64" => {
                if let Ok(v) = serde_json::from_value::<i64>(value) {
                    self.impl_.set_prop_int64(v);
                }
            }
            "propFloat" => {
                if let Ok(v) = serde_json::from_value::<f32>(value) {
                    self.impl_.set_prop_float(v);
                }
            }
            "propFloat32" => {
                if let Ok(v) = serde_json::from_value::<f32>(value) {
                    self.impl_.set_prop_float32(v);
                }
            }
            "propFloat64" => {
                if let Ok(v) = serde_json::from_value::<f64>(value) {
                    self.impl_.set_prop_float64(v);
                }
            }
            "propString" => {
                if let Ok(v) = serde_json::from_value::<String>(value) {
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
        tracing::info!("SimpleInterface linked");
    }

    fn olink_unlinked(
        &self,
        _object_id: &str,
    ) {
        tracing::info!("SimpleInterface unlinked");
    }

    fn olink_collect_properties(&self) -> Value {
        json!({
            "propBool": self.impl_.prop_bool(),
            "propInt": self.impl_.prop_int(),
            "propInt32": self.impl_.prop_int32(),
            "propInt64": self.impl_.prop_int64(),
            "propFloat": self.impl_.prop_float(),
            "propFloat32": self.impl_.prop_float32(),
            "propFloat64": self.impl_.prop_float64(),
            "propString": self.impl_.prop_string()
        })
    }
}
