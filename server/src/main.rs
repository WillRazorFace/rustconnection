mod menu;
use std::io;
use std::process;
use std::{io::Write, ops::Deref, sync::Arc, thread::sleep, time::Duration};
use tokio::{net::TcpListener, sync::Mutex};
use util;
mod core;

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:2202";
    let server = TcpListener::bind(addr).await.unwrap();
    let clients: util::Clients = Arc::new(Mutex::new(Vec::new()));

    // Start handler
    tokio::spawn(core::handle_clients(server, clients.clone()));
    println!("[+] Now listening in {} [+]\n", addr);

    // Start menu
    menu::main_menu(clients).await;
}
