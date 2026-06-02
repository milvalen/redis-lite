use std::sync::{Arc, Mutex};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

use crate::command::Command;
use crate::store::Store;

pub async fn run(addr: &str, store: Arc<Mutex<Store>>) -> anyhow::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    println!("redis-lite listening on {addr}");
    println!("try: telnet 127.0.0.1 6379");

    loop {
        let (socket, peer) = listener.accept().await?;
        let store = Arc::clone(&store);
        println!("+ {peer} connected");

        tokio::spawn(async move {
            if let Err(e) = handle(socket, store).await {
                eprintln!("! connection error: {e}");
            }
            println!("- {peer} disconnected");
        });
    }
}

async fn handle(
    socket: tokio::net::TcpStream,
    store: Arc<Mutex<Store>>,
) -> anyhow::Result<()> {
    let (reader, mut writer) = socket.into_split();
    let mut lines = BufReader::new(reader).lines();

    while let Some(line) = lines.next_line().await? {
        if line.trim().is_empty() {
            continue;
        }

        let response = match Command::parse(&line) {
            Command::Set { key, value } => {
                // TODO: lock store, call set(), return "+OK\n" or "-ERR ...\n"
                // let mut s = store.lock().unwrap();
                // match s.set(key, value) {
                //     Ok(_)  => "+OK\n".to_string(),
                //     Err(e) => format!("-ERR {e}\n"),
                // }
                "-ERR set not implemented\n".to_string()
            }

            Command::Get { key } => {
                // TODO: lock store, call get()
                // return the value as "$value\n", or "$nil\n" if missing
                // let s = store.lock().unwrap();
                // match s.get(&key) {
                //     Some(v) => format!("${v}\n"),
                //     None    => "$nil\n".to_string(),
                // }
                "-ERR get not implemented\n".to_string()
            }

            Command::Del { key } => {
                // TODO: lock store, call del()
                // return ":1\n" if key existed, ":0\n" if not
                // let mut s = store.lock().unwrap();
                // match s.del(&key) {
                //     Ok(true)  => ":1\n".to_string(),
                //     Ok(false) => ":0\n".to_string(),
                //     Err(e)    => format!("-ERR {e}\n"),
                // }
                "-ERR del not implemented\n".to_string()
            }

            Command::Unknown(raw) => {
                format!("-ERR unknown command '{raw}'\n")
            }
        };

        writer.write_all(response.as_bytes()).await?;
    }

    Ok(())
}
