use std::path::Path;
use std::sync::Arc;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::Mutex,
};

pub type Clients = Arc<Mutex<Vec<TcpStream>>>;

pub async fn read_file(path: String) -> Result<Vec<u8>, ()> {
    if Path::new(path.as_str()).exists() == true {
        let mut file = File::open("path").await.unwrap();
        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer).await.unwrap();

        Ok(buffer)
    } else {
        Err(())
    }
}

pub async fn write_file(path: String, data: Vec<u8>) -> Result<(), ()> {
    if Path::new(path.as_str()).exists() == true {
        let mut file = File::open("path").await.unwrap();

        file.write_all(&data).await.unwrap();

        Ok(())
    } else {
        Err(())
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
