use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

const PORT: &str = "2202";

#[tokio::main]
async fn main() {
    let server = TcpListener::bind(format!("0.0.0.0:{}", PORT))
        .await
        .unwrap();

    let (mut client, addr) = server.accept().await.unwrap();
    let (read, mut write) = client.split();

    let mut reader = BufReader::new(read);
    let mut line = String::new();

    println!("Connection from ({})!", addr);

    loop {
        reader.read_line(&mut line).await.unwrap();
        write.write_all(line.as_bytes()).await.unwrap();
    }
}
