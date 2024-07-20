import socket
import threading
import sys

def scan_port(ip, port):
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    socket.setdefaulttimeout(1)
    result = sock.connect_ex((ip, port))
    if result == 0:
        return port
    sock.close()

def scan_ports(ip, ports):
    open_ports = []
    threads = []
    for port in ports:
        t = threading.Thread(target=lambda p: open_ports.append(scan_port(ip, p)), args=(port,))
        threads.append(t)
        t.start()
    for t in threads:
        t.join()
    return [p for p in open_ports if p is not None]

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python network_scanner.py <target_ip>")
        sys.exit(1)

    target_ip = sys.argv[1]
    target_ports = range(1, 1025)  # Scan first 1024 ports
    open_ports = scan_ports(target_ip, target_ports)
    print(open_ports)
