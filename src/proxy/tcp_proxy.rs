use std::net::TcpStream;
use std::{io, thread};

pub fn handle_connection(mut client: TcpStream, backend_addr: String) {
    let mut backend = TcpStream::connect(backend_addr.clone()).unwrap();

    let mut client_reader = client.try_clone().unwrap();
    let mut backend_reader = backend.try_clone().unwrap();
    
    println!("Using {}", backend_addr);

    // client -> backend
    thread::spawn(move || {
        io::copy(&mut client_reader, &mut backend).expect("failed client -> backend");
    });

    // backend -> client
    thread::spawn(move || {
        io::copy(&mut backend_reader, &mut client).expect("failed backend -> client")
    });
}