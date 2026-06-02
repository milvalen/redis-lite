mod command;
mod server;
mod store;

use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "127.0.0.1:6379";
    let log_path = "data/store.log";

    std::fs::create_dir_all("data")?;

    let store = store::Store::open(log_path)?;
    let store = Arc::new(Mutex::new(store));

    server::run(addr, store).await
}
