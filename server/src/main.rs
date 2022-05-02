mod gen;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::Mutex,
};

const PORT: &str = "2202";

async fn handle_clients(listener: TcpListener, clients_list: gen::Clients) {
    loop {
        let (client, addr) = listener.accept().await.unwrap();
        clients_list.lock().await.insert(addr.to_string(), client);

        println!("[+] Connection from ({}) [+]", addr);
    }
}

#[tokio::main]
async fn main() {
    let server = TcpListener::bind(format!("0.0.0.0:{}", PORT))
        .await
        .unwrap();
    let clients: gen::Clients = Arc::new(Mutex::new(HashMap::new()));

    tokio::spawn(handle_clients(server, clients.clone()));

    loop {}
}
