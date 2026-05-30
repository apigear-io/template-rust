use apigear::{ApiError, ApiFuture};
use tokio::sync::{broadcast};

pub struct NoPropertiesInterfacePublisher {
    pub sig_void: broadcast::Sender<()>,
    pub sig_bool: broadcast::Sender<(bool,)>,
}

impl Default for NoPropertiesInterfacePublisher {
    fn default() -> Self {
        Self { sig_void: broadcast::Sender::new(16), sig_bool: broadcast::Sender::new(16) }
    }
}

pub trait NoPropertiesInterfaceTrait: Send + Sync {
    fn func_void(&self) -> ApiFuture<'_, Result<(), ApiError>>;

    fn func_bool(
        &self,
        param_bool: bool,
    ) -> ApiFuture<'_, Result<bool, ApiError>>;

    fn publisher(&self) -> &NoPropertiesInterfacePublisher;
}
