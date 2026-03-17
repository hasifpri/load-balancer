use std::sync::Arc;
use std::thread;
use std::time::Duration;
use crate::backend::backend::Backend;
use crate::health::checker;

pub fn start_health_check(backends: Vec<Arc<Backend>>) {

    thread::spawn(move || {
        loop {

            println!("running health check");

            for backend in &backends {
                let status = checker::check_backend(&backend.address);

                backend.set_alive(status);

                if status {
                    println!("{} OK", backend.address);
                }else {
                    println!("{} Down", backend.address);
                }

            }

            thread::sleep(Duration::from_secs(5));
        }
    });
}