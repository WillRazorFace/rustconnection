use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    time::timeout,
};
use util;
// use tokio::io::AsyncBufReadExt

pub async fn handle_clients(listener: TcpListener, clients_list: util::Clients) {
    loop {
        let (client, _addr) = listener.accept().await.unwrap();
        clients_list.lock().await.push(client);
    }
}

pub async fn check_connection(client: &mut TcpStream) -> Result<(), ()> {
    let (read, mut write) = client.split();

    let mut reader = BufReader::new(read);
    let mut buffer = [0u8; 5];

    write.write_all("CHECK_ALIVE".as_bytes()).await.unwrap();

    if let Err(_) = timeout(Duration::from_secs(20), reader.read_exact(&mut buffer)).await {
        return Err(());
    }

    if buffer == "ALIVE".as_bytes() {
        return Ok(());
    } else {
        return Err(());
    }
}
