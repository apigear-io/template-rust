#![allow(dead_code)]
use std::time::Duration;

/// Connect to the nats-server (`NATS_URL` env var, default `127.0.0.1:4222`).
pub async fn connect() -> async_nats::Client {
    let url = std::env::var("NATS_URL").unwrap_or_else(|_| "127.0.0.1:4222".to_string());
    async_nats::connect(&url).await.expect("connect to nats-server")
}

/// Give background subscriptions time to register before exercising them (generous for CI).
pub async fn settle() {
    tokio::time::sleep(Duration::from_millis(300)).await;
}

/// Poll a condition until it holds or a ~6s timeout elapses (generous for CI).
pub async fn wait_until<F: Fn() -> bool>(cond: F) -> bool {
    for _ in 0..300 {
        if cond() {
            return true;
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    cond()
}
