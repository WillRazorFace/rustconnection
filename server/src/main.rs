mod gen;
use std::collections::HashMap;
use std::io;
use std::io::{Read, Write};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use tokio::{net::TcpListener, sync::Mutex};

const PORT: &str = "2202";

async fn handle_clients(listener: TcpListener, clients_list: gen::Clients) {
    loop {
        let (client, addr) = listener.accept().await.unwrap();
        clients_list.lock().await.push(client);
    }
}

#[tokio::main]
async fn main() {
    let server = TcpListener::bind(format!("0.0.0.0:{}", PORT))
        .await
        .unwrap();
    let clients: gen::Clients = Arc::new(Mutex::new(Vec::new()));

    tokio::spawn(handle_clients(server, clients.clone()));

    println!("\nType 'sessions' to see all clients");
    print!(">>> ");
    io::stdout().flush().unwrap();

    let mut command = String::new();

    io::stdin().read_line(&mut command).unwrap();
    println!("");

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

            println!(
                "\n{}",
                clients.lock().await.get(session).unwrap().peer_addr().unwrap()
            );
        }
        _ => {}
    }
}
