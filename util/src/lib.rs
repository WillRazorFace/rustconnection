use std::sync::Arc;
use tokio::{net::TcpStream, sync::Mutex};

pub type Clients = Arc<Mutex<Vec<TcpStream>>>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
