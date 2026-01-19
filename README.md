
### Distributed Sequencer & Leader Election POC

Q. Split Brain Issue:

    POC to address the splut brain issue during Secondary’s election. Leash a proprietary solution to provide access over the Engine Primary role from a secondary when primary is down. 

    Expected:
        1. High performant gateway proxying the primary.
        2. Leash program should Identify the nodes health. 
        3. Secondary should perform an attempt to get the access to the leash.
        4. New Leader should be updated to the gateway.
        5. Gateway should start sending the message to new Primary


Implementations: 

ArcSwap: 
A high-speed "address book" that lets the Gateway instantly
switch to the new Leader’s IP without ever locking or slowing down 
incoming traffic.

ETCD lock() lets a client acquire a distributed lock so only one process can access a shared resource at a time.


Proof of Concepts in POC:
    1. Prove the Primary exists:
        `etcdctl get engine/leader`
    2. Prove Health Monitoring (Watch)
        `etcdctl watch engine/leader`



We got: 

```
1. No split brain
2. No duplicate execution
3. No missing data after failover
```


Diagram:

```
                    Gateway 


            primary (node 1)          secondary (s1) (node 2)


                    secondary (s2) (node 2)



                    Leash Gateway

                

                                
`````
                
Workflow: 

    Primary Job: 
        1. Write in log file first
        2. Forward to secondary. 

    Secondary:
        1. Write in log file first
        2. Forward to secondary.



Check to know any duplicates on log file:
cat central_order_book.log | awk -F'[][]' '{print $2}' | sort | uniq -d


How to run the project:
```
    cargo run -- gateway 8080
    cargo run -- node 127.0.0.1:8081 P1
    cargo run -- node 127.0.0.1:8082 S1
    cargo run -- node 127.0.0.1:8083 S2
```

Remove existing log data and etcd leader data:
```
    etcdctl del "" --prefix
    rm central_order_book.log
```


Push Data to Gateway:
``` python3 producer.py ```

