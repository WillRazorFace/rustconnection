use std::collections::HashMap;
use std::sync::Arc;
use tokio::{net::TcpStream, sync::Mutex};

pub type Clients = Arc<Mutex<HashMap<String, TcpStream>>>;
