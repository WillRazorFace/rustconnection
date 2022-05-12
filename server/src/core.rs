use std::{str::from_utf8, sync::Arc, time::Duration};
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
        let (mut client, _addr) = listener.accept().await.unwrap();

        let mut buffer = util::receive_with_delimiter(&mut client).await;

        let device_info = from_utf8(&buffer).unwrap().lines().collect::<Vec<&str>>();

        let (os, username, device_name) = (device_info[0], device_info[1], device_info[2]);

        let client = util::Client::new(
            client,
            os.to_string(),
            username.to_string(),
            device_name.to_string(),
        );

        clients_list.lock().await.push(client);
    }
}

pub async fn check_connection(client: &mut util::Client) -> Result<(), ()> {
    let (read, mut write) = client.stream.split();

    let mut reader = BufReader::new(read);
    let mut buffer = [0u8; 5];

    write.write_all("CHECK_ALIVE\r".as_bytes()).await.unwrap();

    if let Err(_) = timeout(Duration::from_secs(20), reader.read_exact(&mut buffer)).await {
        return Err(());
    }

    if buffer == "ALIVE".as_bytes() {
        return Ok(());
    } else {
        return Err(());
    }
}
