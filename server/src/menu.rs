use std::io::Write;
use std::{io, ops::Deref, thread, time};
use tokio::sync::Mutex;
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
                for (index, client) in client_list.lock().await.deref().iter().enumerate() {
                    println!("[{}] {}", index, client.peer_addr().unwrap());
                }
            }
            _ => {}
        }
    }
}
