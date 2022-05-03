mod gen;
use std::io;
use std::{io::Write, ops::Deref, sync::Arc, time::Duration};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::Mutex,
    time::timeout,
};

const PORT: &str = "2202";

async fn handle_clients(listener: TcpListener, clients_list: gen::Clients) {
    loop {
        let (client, _addr) = listener.accept().await.unwrap();
        clients_list.lock().await.push(client);
    }
}

async fn check_connection(client: &mut TcpStream) -> Result<(), ()> {
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

#[tokio::main]
async fn main() {
    let server = TcpListener::bind(format!("0.0.0.0:{}", PORT))
        .await
        .unwrap();
    let clients: gen::Clients = Arc::new(Mutex::new(Vec::new()));

    // Start handler
    tokio::spawn(handle_clients(server, clients.clone()));

    println!("\nType 'sessions' to see all clients");
    print!(">>> ");
    io::stdout().flush().unwrap();

    let mut command = String::new();

    io::stdin().read_line(&mut command).unwrap();
    println!("");

    // If command is sessions, shows clients
    match command.as_str().trim() {
        "sessions" => {
            for (index, client) in clients.lock().await.deref().iter().enumerate() {
                println!("[{}] {}", index, client.peer_addr().unwrap());
            }

            println!("\nSelect session");
            print!(">>> ");
            io::stdout().flush().unwrap();

            let mut session = String::new();

            io::stdin().read_line(&mut session).unwrap();
            println!("");

            let session: usize = match session.trim().parse() {
                Ok(session) => session,
                Err(e) => panic!("Conversion error: {}", e),
            };

            let mut client = clients.lock().await.remove(session);

            // Check if client still connected
            match check_connection(&mut client).await {
                Ok(_e) => println!("[+] Still connected [+]"),
                Err(_) => drop(client),
            }

            for (index, client) in clients.lock().await.deref().iter().enumerate() {
                println!("[{}] {}", index, client.peer_addr().unwrap());
            }
        }
        _ => {}
    }
}
