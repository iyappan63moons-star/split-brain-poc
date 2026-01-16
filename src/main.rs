mod leash;
mod gateway;

use std::env;

#[tokio::main]
async fn main() {

    let args: Vec<String> = env::args().collect();

    let etcd_url = "http://127.0.0.1:2379";

    if args.len() < 2 {
        println!("Usage:");
        println!("Gateway: cargo run -- gateway 8080");
        println!("Leash:   cargo run -- leash node-1 8081");
        return;
    }

    
    match args[1].as_str() {

        "gateway" => {
            let port = args[2].parse().expect("Invalid port");
            gateway::run_gateway(etcd_url, port).await;
        }
        "leash" => {
            let node_id = args[2].clone();
            let engine_port = args[3].parse().expect("Invalid engine port");
            leash::run_leash(node_id, etcd_url, engine_port).await;
        }
        _ => println!("Unknown command"),
    }




}



