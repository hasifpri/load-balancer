mod backend;
mod balancer;
mod proxy;
mod health;
mod config;

use std::net::TcpListener;
use std::sync::Arc;
use balancer::round_robin::RoundRobin;
use health::runner;
use crate::backend::backend::Backend;

fn main() {

    // Load Config
    let config_data = config::config::load_config();

    // addr
    let addr = format!("0.0.0.0:{}", config_data.port);
    let listener = TcpListener::bind(addr).expect("can't bind port");

    // load backends
    let backends: Vec<Arc<Backend>> = config_data.backends.servers.into_iter().map(|addr| Arc::new(Backend::new(addr))).collect();
    let balancer = Arc::new(RoundRobin::new(backends.clone()));
    
    // Running Checker
    runner::start_health_check(backends.clone());

    println!("load balancer running in port {}", config_data.port);

    for stream in listener.incoming() {

        match stream {
            Ok(stream) => {

                let balancer = balancer.clone();

                std::thread::spawn(move || {

                    let balancer = balancer.next();
                    proxy::tcp_proxy::handle_connection(stream, balancer.unwrap());
                });
            }

            Err(e) => {
                println!("connection error {}", e);
            }
        }

    }
}


