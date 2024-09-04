use std::io;
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::Duration;

fn scan_port(ip: &str, port: u16, timeout: Duration, tx: Sender<u16>) {
    let address = format!("{}:{}", ip, port);
    if let Ok(mut addr) = address.to_socket_addrs() {
        if let Some(sock_addr) = addr.next() {
            if TcpStream::connect_timeout(&sock_addr, timeout).is_ok() {
                tx.send(port).unwrap();
            }
        }
    }
}

fn main() -> io::Result<()> {
    let ip = "127.0.0.1"; // Change this to the target IP address
    let start_port = 1;
    let end_port = 1024;
    let timeout = Duration::from_secs(1);
    let thread_count = 100; // Number of threads to use for scanning

    let (tx, rx) = mpsc::channel();

    for i in (start_port..=end_port).step_by(thread_count) {
        let mut handles = vec![];

        for port in i..i + thread_count {
            if port > end_port {
                break;
            }

            let tx = tx.clone();
            let ip = ip.to_string();
            let timeout = timeout.clone();

            // Convert `port` from `usize` to `u16`
            let port = port as u16;

            let handle = thread::spawn(move || {
                scan_port(&ip, port, timeout, tx);
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    drop(tx); // Close the sending end of the channel

    println!("Open ports:");

    for port in rx {
        println!("Port {} is open", port);
    }

    Ok(())
}
