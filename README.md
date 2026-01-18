
Split Brain Issue:

    POC to address the splut brain issue during Secondary’s election. Leash a proprietary solution to provide access over the Engine Primary role from a secondary when primary is down. 

    Expected:

        1. High performant gateway proxying the primary.
        2. Leash program should Identify the nodes health. 
        3. Secondary should perform an attempt to get the access to the leash.
        4. New Leader should be updated to the gateway.
        5. Gateway should start sending the message to new Primary



ETCD Methods:

Client	    =>	Connecting to the "Source of Truth".
Compare	    =>	Checking if a Leader already exists.
TxnOp	    =>	Preparing to write the new Leader's IP.
Txn     	=>	Ensuring the election is fair and has only one winner.
PutOptions	=>	Setting the 5-second health check (Lease).



ArcSwap: 
A high-speed "address book" that lets the Gateway instantly
switch to the new Leader’s IP without ever locking or slowing down 
incoming traffic.

copy_bidirectional: 
A high-efficiency "digital bridge" that automatically pipes data back and forth between the 
User and the Primary Engine at maximum speed.


How fixed no split brain issue?:
Atomic Nature of the election and the Fencing of the gateway.


Proof of Concepts in POC:
    1. Prove the Primary exists:
        `etcdctl get engine/leader`
    2. Prove Health Monitoring (Watch)
        `etcdctl watch engine/leader`
    



Gateway 

-

primary 

secondary  (s1)

secondary (s2)

(.log)

system signal interval

sequencer id


primary job: 
    1. write in log file first
    2. forward to secondary. 

secondary:
    1. write in log file first
    2. forward to secondary. 


cat central_order_book.log | awk -F'[][]' '{print $2}' | sort | uniq -d


no split brain
no duplicate execution
no missing data after failover


cargo run -- gateway 8080
cargo run -- node 127.0.0.1:8081 P1
cargo run -- node 127.0.0.1:8082 S1
cargo run -- node 127.0.0.1:8083 S2

python3 producer.py