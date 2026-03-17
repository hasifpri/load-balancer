use std::net::TcpStream;
use std::time::Duration;

pub fn check_backend(addr: &str) -> bool {
    
    match TcpStream::connect_timeout(
        &addr.parse().unwrap(),
        Duration::from_secs(2),
    ) { 
        Ok(_) => true,
        Err(_) => false,
    }
}
