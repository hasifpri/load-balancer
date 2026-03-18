use std::sync::Arc;
use std::thread;
use std::time::Duration;
use crate::backend::backend::Backend;
use crate::health::checker;

pub fn start_health_check(backends: Vec<Arc<Backend>>) {

    println!("background process running health check");

    thread::spawn(move || {
        loop {

            for backend in &backends {
                let status = checker::check_backend(&backend.address);

                // Check OK
                if status {

                    // If OK
                    backend.set_alive(status);
                    backend.reset_fail();
                }else {

                    // If not OK increase fail
                    backend.inc_fail();
                }

                // Set Instance Backend To Deactivate
                if backend.get_fail() > 2 {
                    backend.set_alive(status);
                }
            }

            thread::sleep(Duration::from_secs(5));
        }
    });
}

pub fn start_monitoring(backends: Vec<Arc<Backend>>) {

    println!("Background Monitoring Backend Instance");

    thread::spawn(move || {
        loop {

            println!("Current Monitoring Instance Backend");

            for backend in &backends {
                println!("Instance {}: Status {}; Current_Fail: {}; Current_Handle: {}", backend.address, backend.get_alive(), backend.get_fail(), backend.get_conn())
            }

            thread::sleep(Duration::from_secs(10));
        }
    });

}