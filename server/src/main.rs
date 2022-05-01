use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

const PORT: &str = "2202";

#[tokio::main]
async fn main() {
    let server = TcpListener::bind(format!("0.0.0.0:{}", PORT))
        .await
        .unwrap();

    let (mut client, addr) = server.accept().await.unwrap();

    println!("Connection from ({})!", addr);

    loop {
        let mut buffer = [0u8; 1024];

        let bytes_read = client.read(&mut buffer).await.unwrap();

        client.write_all(&buffer[..bytes_read]).await.unwrap();
    }
}
