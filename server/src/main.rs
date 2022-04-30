use std::io;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str::from_utf8;

const PORT: &str = "2202";

fn send_data(mut stream: &TcpStream, data: String) {
    stream.write_all(data.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn receive_data(mut stream: &TcpStream) -> io::Result<String> {
    let mut buffer = [0u8; 1024];

    stream.read(&mut buffer).unwrap();
    stream.flush().unwrap();

    Ok(String::from(from_utf8(&buffer).unwrap()))
}

fn handler(stream: TcpStream) {
    let addr = stream.local_addr().unwrap();

    println!("Connectd with ({}). Enjoy!\n", addr);

    loop {
        print!("(You) > ");
        io::stdout().flush().unwrap();

        let mut data = String::new();

        io::stdin().read_line(&mut data).unwrap();
        io::stdout().flush().unwrap();

        send_data(&stream, data);

        let response: String = receive_data(&stream).unwrap();

        print!("({}) > {}", addr, response);
        io::stdout().flush().unwrap();
    }
}

fn main() {
    let server: TcpListener = TcpListener::bind(format!("0.0.0.0:{}", PORT)).unwrap();

    for client in server.incoming() {
        let client = client.unwrap();

        handler(client);
    }
}
