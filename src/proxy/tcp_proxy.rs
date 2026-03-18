use std::net::{Shutdown, TcpStream};
use std::{io, thread};

pub fn handle_connection(client: TcpStream, backend_addr: &str) -> Result<(), io::Error> {
    let backend = TcpStream::connect(backend_addr)?;
    
    // clone bidirectional
    let mut client_reader = client.try_clone()?;
    let mut backend_writer = backend.try_clone()?;

    let mut backend_reader = backend.try_clone()?;
    let mut client_writer = client.try_clone()?;

    // client -> backend
    let t1 = thread::spawn(move || {
        io::copy(&mut client_reader, &mut backend_writer)
    });

    // backend -> client
    let t2 = thread::spawn(move || {
        io::copy(&mut backend_reader, &mut client_writer)
    });
    
    // wait until done
    let r1 = t1.join().unwrap();
    let r2 = t2.join().unwrap();

    let _ = backend.shutdown(Shutdown::Both);
    let _ = client.shutdown(Shutdown::Both);

    match (r1, r2) {
        (Ok(_), Ok(_)) => Ok(()),
        _ => Err(io::Error::new(
            io::ErrorKind::Other,
            "proxy error",
        )),
    }
    
}