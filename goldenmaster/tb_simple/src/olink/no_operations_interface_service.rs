use crate::api::no_operations_interface::NoOperationsInterfaceTrait;
use objectlink_core::traits::{IRemoteNode, ObjectSource};
use objectlink_core::types::Name;
use serde_json::{json, Value};
use std::sync::Arc;

/// OLink service adapter for NoOperationsInterface.
/// Bridges a local implementation to the OLink protocol by implementing ObjectSource.
pub struct NoOperationsInterfaceOlinkService {
    impl_: Arc<dyn NoOperationsInterfaceTrait>,
}

impl NoOperationsInterfaceOlinkService {
    pub fn new(impl_: Arc<dyn NoOperationsInterfaceTrait>) -> Self {
        Self { impl_ }
    }
}

impl ObjectSource for NoOperationsInterfaceOlinkService {
    fn olink_object_name(&self) -> &str {
        "tb.simple.NoOperationsInterface"
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
        tracing::info!("NoOperationsInterface linked");
    }

    fn olink_unlinked(
        &self,
        _object_id: &str,
    ) {
        tracing::info!("NoOperationsInterface unlinked");
    }

    fn olink_collect_properties(&self) -> Value {
        json!({
            "propBool": self.impl_.prop_bool(),
            "propInt": self.impl_.prop_int()
        })
    }
}
