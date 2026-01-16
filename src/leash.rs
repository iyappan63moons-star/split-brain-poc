use etcd_client::{Client, Compare, CompareOp, PutOptions, Txn, TxnOp};
use std::time::Duration;
use tokio::time::sleep;


pub async fn run_leash(node_id: String, etcd_url: &str, engine_port: u16) {


    let mut client = Client::connect([etcd_url], None).await.expect("etcd down");

    let node_address = format!("127.0.0.1:{}", engine_port);


    loop {

        let lease_res = client.lease_grant(5, None).await.expect("Failed to get lease");

        let lease_id = lease_res.id();


        let txn = Txn::new()
            .when(vec![Compare::version("engine/leader", CompareOp::Equal, 0)])
            .and_then(vec![TxnOp::put(
                "engine/leader",
                node_address.clone(),
                Some(PutOptions::new().with_lease(lease_id)),
            )]);

        
        if let Ok(resp) = client.txn(txn).await {

            if resp.succeeded() {

                println!("SUCCESS: {} is now PRIMARY (Routing to {})", node_id, node_address);
                
                let (mut keeper, _) = client.lease_keep_alive(lease_id).await.unwrap();

                loop {
                    if let Err(_) = keeper.keep_alive().await {
                        println!("Leash broken for {}. Stepping down...", node_id);
                        break;
                    }
                    sleep(Duration::from_secs(2)).await;
                }


            } else {

                println!("Node {} is SECONDARY. Leader already exists. Retrying...", node_id);
                sleep(Duration::from_secs(2)).await;
                
            }
        }
    }



}