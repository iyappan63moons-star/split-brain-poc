mod gateway;
mod node;

#[tokio::main]
async fn main() {
    
    let args: Vec<String> = std::env::args().collect();

    let etcd_url = "localhost:2379";

    if args.len() < 2 {
        println!("Usage:");
        println!("  cargo run -- gateway <port>");
        println!("  cargo run -- node <IP:PORT> <NAME>");
        return;
    }

    match args[1].as_str() {

        "gateway" => {
            let port = args.get(2).and_then(|p| p.parse().ok()).unwrap_or(8080);
            gateway::run_gateway(etcd_url, port).await;
        },
        "node" => {
            if args.len() < 4 {
                println!("Error: Node requires address and name.");
                return;
            }

            let addr = &args[2];
            let name = &args[3];
            node::run_node(etcd_url, addr, name).await;
        },
        _ => println!("Unknown command. Use 'gateway' or 'node'."),

    }
}