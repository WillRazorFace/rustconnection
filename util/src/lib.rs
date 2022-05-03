use std::path::Path;
use std::sync::Arc;
use tokio::{fs::File, io::AsyncReadExt, net::TcpStream, sync::Mutex};

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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
