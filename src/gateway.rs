use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use arc_swap::ArcSwap;
use etcd_client::Client;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static GLOBAL_SEQ: AtomicU64 = AtomicU64::new(1000);

pub async fn run_gateway(etcd_url: &str, port: u16) {

    let client = Client::connect([etcd_url], None).await.unwrap();

    let state = Arc::new(ArcSwap::from_pointee("0.0.0.0:0".to_string()));

    let state_clone = state.clone();

    tokio::spawn(async move {
        let mut watch_client = client.clone();
        let (_, mut stream) = watch_client.watch("engine/leader", None).await.unwrap();
        while let Ok(Some(resp)) = stream.message().await {
            for event in resp.events() {
                if let Some(kv) = event.kv() {
                    let addr = kv.value_str().unwrap().to_string();
                    println!("WATCHER: New Primary detected at -> {}", addr);
                    state_clone.store(Arc::new(addr));
                }
            }
        }
    });


    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await.unwrap();

    println!("GATEWAY: Ready at 127.0.0.1:{}", port);

    loop {

        let (mut inbound, _) = listener.accept().await.unwrap();
        let state_handle = state.clone();

        tokio::spawn(async move {
            let mut buf = [0u8; 1024];
            let n = inbound.read(&mut buf).await.unwrap_or(0);
            if n == 0 { return; }
            
            let data = String::from_utf8_lossy(&buf[..n]).trim().to_string();
            let seq_id = GLOBAL_SEQ.fetch_add(1, Ordering::SeqCst);
            let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
            let payload = format!("[SEQ:{}][TS:{}] DATA:{}\n", seq_id, ts, data);

            // retry loop (data send)
            loop {
                let target = state_handle.load();
                if **target != "0.0.0.0:0" {
                    if let Ok(mut outbound) = TcpStream::connect(target.as_str()).await {
                        if outbound.write_all(payload.as_bytes()).await.is_ok() {
                            return; 
                        }
                    }
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            }
        });

    }




}