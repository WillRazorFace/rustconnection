use std::env;
use std::path::Path;
use std::process;
use std::str::from_utf8;
use std::sync::Arc;
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, AsyncRead, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
    sync::Mutex,
};

pub struct Client {
    pub stream: TcpStream,
    pub os: String,
    pub username: String,
    pub device_name: String,
}

impl Client {
    pub fn new(stream: TcpStream, os: String, username: String, device_name: String) -> Self {
        Client {
            stream: stream,
            os: os,
            username: username,
            device_name: device_name,
        }
    }
}

pub type Clients = Arc<Mutex<Vec<Client>>>;

pub async fn read_file(path: &str) -> Result<Vec<u8>, &'static str> {
    if Path::new(path).is_file() == true {
        let mut file = File::open(path).await.unwrap();
        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer).await.unwrap();

        Ok(buffer)
    } else {
        Err("Invalid path")
    }
}

pub async fn upload_file(path: &str, mut stream: TcpStream) -> Result<(), &'static str> {
    if let Ok(data) = read_file(path).await {
        let filename = format!(
            "{}\n",
            Path::new(path).file_name().unwrap().to_str().unwrap()
        );

        stream.write_all(filename.as_bytes()).await.unwrap();
        stream.write_all(&data).await.unwrap();

        Ok(())
    } else {
        Err("Can't read file")
    }
}

pub async fn download_file(path: &str, mut stream: TcpStream) -> Result<(), &'static str> {
    let (read, _write) = stream.split();
    let mut reader = BufReader::new(read);
    let mut filename = String::new();

    let current_path = env::current_dir().unwrap();

    let path = match path {
        "" => current_path.to_str().unwrap(),
        _ => path,
    };

    reader.read_line(&mut filename).await.unwrap();

    if let Ok(mut file) = File::create(format!("{}/{}", path, filename.trim())).await {
        let mut buffer: Vec<u8> = Vec::new();

        reader.read_to_end(&mut buffer).await.unwrap();

        file.write_all(&buffer).await.unwrap();

        Ok(())
    } else {
        Err("Cant't create file")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub async fn send_with_delimiter(stream: &mut TcpStream, data: &[u8]) {
    stream.write_all(data).await.unwrap();
    stream.write_all("\r".as_bytes()).await.unwrap();
}

pub async fn receive_with_delimiter(stream: &mut TcpStream) -> Vec<u8> {
    let (read, _write) = stream.split();
    let mut reader = BufReader::new(read);
    let mut buffer: Vec<u8> = Vec::new();

    reader.read_until(b'\r', &mut buffer).await.unwrap();

    buffer
}
