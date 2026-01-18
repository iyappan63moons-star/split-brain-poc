use etcd_client::{Client, Compare, CompareOp, Txn, TxnOp, PutOptions, LockOptions};
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::fs::OpenOptions;
use std::time::Duration;

// Transaction Method

pub async fn run_node_transactiom_method(etcd_url: &str, node_addr: &str, node_name_ref: &str) {

    let mut client = Client::connect([etcd_url], None).await.unwrap();
    let node_name = node_name_ref.to_string();
    let leader_key = "engine/leader";

    loop {

        let lease_res = client.lease_grant(10, None).await.unwrap();
        let lease_id = lease_res.id();

        println!("{} (Standby): Checking for active Primary...", node_name);

      
        let txn = Txn::new()
            .when(vec![Compare::version(leader_key, CompareOp::Equal, 0)])
            .and_then(vec![TxnOp::put(leader_key, node_addr, Some(PutOptions::new().with_lease(lease_id)))]);

        let txn_res = client.txn(txn).await.unwrap();

        if txn_res.succeeded() {
            
            println!(">>> {} WON ELECTION!.", node_name);

            let (mut keeper, _) = client.lease_keep_alive(lease_id).await.unwrap();

            // let name_clone = node_name.clone();

            tokio::spawn(async move {
                loop {
                    if let Err(_) = keeper.keep_alive().await { break; }
                    tokio::time::sleep(Duration::from_secs(2)).await;
                }
            });

            let listener = TcpListener::bind(node_addr).await.unwrap();

            while let Ok((mut socket, _)) = listener.accept().await {

                let mut buf = [0u8; 1024];

                if let Ok(n) = socket.read(&mut buf).await {

                    let msg = String::from_utf8_lossy(&buf[..n]);
                    let mut file = OpenOptions::new().append(true).create(true).open("central_order_book.log").await.unwrap();
                    file.write_all(msg.as_bytes()).await.unwrap();
                    println!("{} log data: {}.", node_name, msg.trim());

                }


            }
        } else {
            // Election Failed. Leader Exists.
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    }
}


// Lock Method

pub async fn run_node(etcd_url: &str, node_addr: &str, node_name_ref: &str) {


    let mut client = Client::connect([etcd_url], None).await.unwrap();
    let node_name = node_name_ref.to_string();

    loop {

        let lease_res = client.lease_grant(10, None).await.unwrap();
        let lease_id = lease_res.id();

      
        let (mut keeper, _) = client.lease_keep_alive(lease_id).await.unwrap();
        tokio::spawn(async move {
            loop {
                let _ = keeper.keep_alive().await;
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        });

        println!("{} (Standby): Waiting for lock...", node_name);

        
        let mut lock_client = client.lock_client();

        if let Ok(_) = lock_client.lock("oms_global_lock", Some(LockOptions::new().with_lease(lease_id))).await {
            
            println!(">>> {} WON ELECTION!", node_name);
            
            client.put("engine/leader", node_addr, None).await.unwrap();

            let listener = TcpListener::bind(node_addr).await.unwrap();
            while let Ok((mut socket, _)) = listener.accept().await {
                let mut buf = [0u8; 1024];
                if let Ok(n) = socket.read(&mut buf).await {
                    let msg = String::from_utf8_lossy(&buf[..n]);
                    println!("{} log data: {}.", node_name, msg.trim());
                    
                    let mut file = OpenOptions::new().append(true).create(true).open("central_order_book.log").await.unwrap();
                    file.write_all(msg.as_bytes()).await.unwrap();
                }
            }
        }


    }
}