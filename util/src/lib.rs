use std::path::Path;
use std::sync::Arc;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::Mutex,
};

pub type Clients = Arc<Mutex<Vec<TcpStream>>>;

pub async fn read_file(path: &str) -> Result<Vec<u8>, &'static str> {
    if Path::new(path).exists() == true {
        let mut file = File::open(path).await.unwrap();
        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer).await.unwrap();

        Ok(buffer)
    } else {
        Err("Invalid directory")
    }
}

pub async fn upload_file(path: &str, mut stream: TcpStream) -> Result<(), &'static str> {
    if let Ok(data) = read_file(path).await {
        stream.write_all(&data).await.unwrap();

        Ok(())
    } else {
        Err("Can't read file")
    }
}

pub async fn download_file(path: &str, mut stream: TcpStream) -> Result<(), &'static str> {
    if let Ok(mut file) = File::create(path).await {
        let mut buffer: Vec<u8> = Vec::new();

        stream.read_to_end(&mut buffer).await.unwrap();

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
