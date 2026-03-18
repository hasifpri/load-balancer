mod backend;
mod balancer;
mod proxy;
mod health;
mod config;

use std::net::TcpListener;
use std::sync::Arc;
// use balancer::round_robin::RoundRobin;
use balancer::least_connection::LeastConn;
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
    let balancer = Arc::new(LeastConn::new(backends.clone()));
    
    // Running Checker
    runner::start_health_check(backends.clone());

    // Running Monitor
    runner::start_monitoring(backends.clone());

    println!("load balancer running in port {}", config_data.port);

    for stream in listener.incoming() {

        match stream {
            Ok(stream) => {

                let balancer = balancer.clone();

                std::thread::spawn(move || {


                    // Check Backend Exists
                    if let Some(backend) = balancer.next_least_conn() {

                        // Check If Backend down
                        if !backend.is_alive() {
                            println!("backend suddenly down, please retry again");
                            return;
                        }

                        // Increase Using
                        backend.inc_conn();

                        proxy::tcp_proxy::handle_connection(stream, &backend.address);

                        backend.dec_conn();
                    }else{

                        println!("NO BACKEND AVAIl!!!")
                    }

                });
            }

            Err(e) => {
                println!("connection error {}", e);
            }
        }

    }
}