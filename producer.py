import socket
import time

def send_orders():
    
    order_id = 1
    gateway_ip = '127.0.0.1'
    gateway_port = 8080

    print(f"Starting continuous order feed to {gateway_ip}:{gateway_port}...")
    
    while True:
        try:
            with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
                s.settimeout(2)
                s.connect((gateway_ip, gateway_port))
                
                message = f"MSG_{order_id}"
                s.sendall(message.encode())
                
                print(f"Sent: {message}")
                order_id += 1
                
        except Exception as e:
            print(f"Gateway connection issue: {e}. Retrying...")
        
        time.sleep(0.5)

if __name__ == "__main__":
    send_orders()