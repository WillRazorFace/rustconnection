use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;

const HOST: &str = "localhost";
const PORT: &str = "2202";

fn receive_data(mut stream: &TcpStream) -> io::Result<String> {
    let mut buffer = [0u8; 1024];

    stream.read(&mut buffer).unwrap();
    stream.flush().unwrap();

    Ok(String::from(from_utf8(&buffer).unwrap()))
}

fn send_data(mut stream: &TcpStream, data: String) {
    stream.write_all(data.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    match TcpStream::connect(format!("{}:{}", HOST, PORT)) {
        Ok(stream) => {
            let addr = stream.local_addr().unwrap();

            println!("Connected in ({})! Enjoy.\n", addr);
            loop {
                let response: String = receive_data(&stream).unwrap();

                print!("({}) > {}", addr, response);
                io::stdout().flush().unwrap();

                print!("(You) > ");
                io::stdout().flush().unwrap();

                let mut data = String::new();

                io::stdin().read_line(&mut data).unwrap();
                io::stdout().flush().unwrap();

                send_data(&stream, data);
            }
        }
        Err(e) => {
            println!("Bad connection: {}", e);
        }
    }
}
