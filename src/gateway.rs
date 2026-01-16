use arc_swap::ArcSwap;
use etcd_client::Client;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};


pub async fn run_gateway(etcd_url: &str, listen_port: u16) {

    let client = Client::connect([etcd_url], None).await.unwrap();

    let state = Arc::new(ArcSwap::from_pointee("0.0.0.0:0".to_string()));

    let state_clone = state.clone();

    let mut watch_client = client.clone();

    tokio::spawn(async move {


        let (_watcher, mut stream) = watch_client.watch("engine/leader", None).await.unwrap();
        println!("Gateway: Waiting for leader election...");
        
        while let Ok(Some(resp)) = stream.message().await {
            for event in resp.events() {
                if let Some(kv) = event.kv() {
                    let addr = kv.value_str().unwrap().to_string();
                    println!("GATEWAY: Traffic re-routed to Primary -> {}", addr);
                    state_clone.store(Arc::new(addr));
                }
            }
        }

        
    });

    let addr = format!("0.0.0.0:{}", listen_port);
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("Gateway high-performance proxy running on {}", addr);

    loop {
        let (mut inbound, _) = listener.accept().await.unwrap();
        let current_target = state.load().clone();
        
        tokio::spawn(async move {
            if *current_target != "0.0.0.0:0" {
                if let Ok(mut outbound) = TcpStream::connect(current_target.as_str()).await {
                    let _ = tokio::io::copy_bidirectional(&mut inbound, &mut outbound).await;
                }
            } else {
                eprintln!("Gateway: No Primary elected yet. Dropping connection.");
            }
        });
    }

    
}