use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

fn main() {
    let target_ip = "scanme.nmap.org"; // Target IP
    let ports = vec![22, 80, 443, 8080, 3306]; // Common ports

    println!("Scanning {}...", target_ip);

    for port in ports {
        if scan_port(target_ip, port) {
            println!("Port {} is open", port);
        }
    }
}

fn scan_port(ip: &str, port: u16) -> bool {
    let addr = format!("{}:{}", ip, port);
    let timeout = Duration::from_secs(1);
    
    if let Ok(mut addrs) = addr.to_socket_addrs() {
        if let Some(socket_addr) = addrs.next() {
            return TcpStream::connect_timeout(&socket_addr, timeout).is_ok();
        }
    }
    false
}
