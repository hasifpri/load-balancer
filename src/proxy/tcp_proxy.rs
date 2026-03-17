use std::net::TcpStream;
use std::{io, thread};

pub fn handle_connection(mut client: TcpStream, backend_addr: &str) {
    let mut backend = TcpStream::connect(backend_addr).unwrap();

    let mut client_reader = client.try_clone().unwrap();
    let mut backend_reader = backend.try_clone().unwrap();

    // client -> backend
    thread::spawn(move || {
        io::copy(&mut client_reader, &mut backend).expect("failed client -> backend");
    });

    // backend -> client
    thread::spawn(move || {
        io::copy(&mut backend_reader, &mut client).expect("failed backend -> client")
    });
}