use std::sync::Arc;
use tokio::{net::TcpStream, sync::Mutex};

pub type Clients = Arc<Mutex<Vec<TcpStream>>>;
