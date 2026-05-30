use apigear::{ApiError, ApiFuture};
use tokio::sync::{broadcast};

pub struct VoidInterfacePublisher {
    pub sig_void: broadcast::Sender<()>,
}

impl Default for VoidInterfacePublisher {
    fn default() -> Self {
        Self { sig_void: broadcast::Sender::new(16) }
    }
}

pub trait VoidInterfaceTrait: Send + Sync {
    fn func_void(&self) -> ApiFuture<'_, Result<(), ApiError>>;

    fn publisher(&self) -> &VoidInterfacePublisher;
}
