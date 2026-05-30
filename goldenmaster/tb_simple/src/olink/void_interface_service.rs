use crate::api::void_interface::VoidInterfaceTrait;
use objectlink_core::traits::{IRemoteNode, ObjectSource};
use objectlink_core::types::Name;
use serde_json::{json, Value};
use std::sync::Arc;

/// OLink service adapter for VoidInterface.
/// Bridges a local implementation to the OLink protocol by implementing ObjectSource.
pub struct VoidInterfaceOlinkService {
    impl_: Arc<dyn VoidInterfaceTrait>,
}

impl VoidInterfaceOlinkService {
    pub fn new(impl_: Arc<dyn VoidInterfaceTrait>) -> Self {
        Self { impl_ }
    }
}

impl ObjectSource for VoidInterfaceOlinkService {
    fn olink_object_name(&self) -> &str {
        "tb.simple.VoidInterface"
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
            "funcVoid" => {
                let rt = tokio::runtime::Handle::try_current().ok().map(|h| tokio::task::block_in_place(|| h.block_on(self.impl_.func_void())));
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
        tracing::info!("VoidInterface linked");
    }

    fn olink_unlinked(
        &self,
        _object_id: &str,
    ) {
        tracing::info!("VoidInterface unlinked");
    }

    fn olink_collect_properties(&self) -> Value {
        json!({})
    }
}
