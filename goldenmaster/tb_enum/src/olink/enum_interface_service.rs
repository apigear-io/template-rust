#[allow(unused_imports)]
use crate::api::data_structs::*;
use crate::api::enum_interface::EnumInterfaceTrait;
use objectlink_core::traits::{IRemoteNode, ObjectSource};
use objectlink_core::types::Name;
use serde_json::{json, Value};
use std::sync::Arc;

/// OLink service adapter for EnumInterface.
/// Bridges a local implementation to the OLink protocol by implementing ObjectSource.
pub struct EnumInterfaceOlinkService {
    impl_: Arc<dyn EnumInterfaceTrait>,
}

impl EnumInterfaceOlinkService {
    pub fn new(impl_: Arc<dyn EnumInterfaceTrait>) -> Self {
        Self { impl_ }
    }
}

impl ObjectSource for EnumInterfaceOlinkService {
    fn olink_object_name(&self) -> &str {
        "tb.enum.EnumInterface"
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
            "func0" => {
                let param_0: Enum0Enum = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func0(param_0))));
                match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                }
            }
            "func1" => {
                let param_0: Enum1Enum = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func1(param_0))));
                match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                }
            }
            "func2" => {
                let param_0: Enum2Enum = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func2(param_0))));
                match rt {
                    Some(Ok(value)) => json!(value),
                    _ => json!(null),
                }
            }
            "func3" => {
                let param_0: Enum3Enum = serde_json::from_value(arr.and_then(|a| a.get(0).cloned()).unwrap_or_default()).unwrap_or_default();
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func3(param_0))));
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
            "prop0" => {
                if let Ok(v) = serde_json::from_value::<Enum0Enum>(value) {
                    self.impl_.set_prop0(v);
                }
            }
            "prop1" => {
                if let Ok(v) = serde_json::from_value::<Enum1Enum>(value) {
                    self.impl_.set_prop1(v);
                }
            }
            "prop2" => {
                if let Ok(v) = serde_json::from_value::<Enum2Enum>(value) {
                    self.impl_.set_prop2(v);
                }
            }
            "prop3" => {
                if let Ok(v) = serde_json::from_value::<Enum3Enum>(value) {
                    self.impl_.set_prop3(v);
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
        tracing::info!("EnumInterface linked");
    }

    fn olink_unlinked(
        &self,
        _object_id: &str,
    ) {
        tracing::info!("EnumInterface unlinked");
    }

    fn olink_collect_properties(&self) -> Value {
        json!({
            "prop0": self.impl_.prop0(),
            "prop1": self.impl_.prop1(),
            "prop2": self.impl_.prop2(),
            "prop3": self.impl_.prop3()
        })
    }
}
