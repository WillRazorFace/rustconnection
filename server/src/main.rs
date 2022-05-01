use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

const PORT: &str = "2202";

async fn handle_clients(listener: TcpListener) {
    loop {
        let (mut client, addr) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let (read, mut write) = client.split();

            let mut reader = BufReader::new(read);
            let mut line = String::new();

            println!("[+] Connection from ({}) [+]", addr);

            loop {
                let bytes_read = match reader.read_line(&mut line).await {
                    Ok(bytes) => bytes,
                    Err(_e) => {
                        write
                            .write_all("Invalid character.".as_bytes())
                            .await
                            .unwrap();
                        continue;
                    }
                };

                if bytes_read == 0 {
                    break;
                }

                write.write_all(&line.as_bytes()).await.unwrap();
                line.clear();
            }
        });
    }
}

#[tokio::main]
async fn main() {
    let server = TcpListener::bind(format!("0.0.0.0:{}", PORT))
        .await
        .unwrap();

    handle_clients(server).await;
}
