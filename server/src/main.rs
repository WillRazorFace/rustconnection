use std::io;
use std::process;
use std::{io::Write, ops::Deref, sync::Arc};
use tokio::{net::TcpListener, sync::Mutex};
use util;
mod core;

const PORT: &str = "2202";

#[tokio::main]
async fn main() {
    let server = TcpListener::bind(format!("0.0.0.0:{}", PORT))
        .await
        .unwrap();
    let clients: util::Clients = Arc::new(Mutex::new(Vec::new()));

    // Start handler
    tokio::spawn(core::handle_clients(server, clients.clone()));

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

            if clients.lock().await.len() < session {
                println!("Incorrect index");
                process::exit(1)
            }

            let mut client = clients.lock().await.remove(session);

            // Check if client still connected
            match core::check_connection(&mut client).await {
                Ok(_e) => println!("[+] Still connected [+]"),
                Err(_) => {
                    drop(client);
                    println!("[-] Connection closed [-]");
                    process::exit(1)
                }
            }

            println!("\nType 'upload' to upload file");
            print!(">>> ");
            io::stdout().flush().unwrap();

            let mut command = String::new();

            io::stdin().read_line(&mut command).unwrap();
            println!("");

            match command.as_str().trim() {
                "upload" => {
                    println!("\nInsert file path (example/path/to/your/file.png)");
                    print!(">>> ");
                    io::stdout().flush().unwrap();

                    let mut path = String::new();

                    io::stdin().read_line(&mut path).unwrap();
                    println!("");

                    util::upload_file(path.as_str().trim(), client)
                        .await
                        .unwrap();
                }
                _ => {}
            }
        }
        _ => {}
    }
}
