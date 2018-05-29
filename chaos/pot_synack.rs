// https://doc.rust-lang.org/std/net/struct.TcpListener.html
// This alerts after SYN and SYN-ACK is done and closes.

use std::net::TcpListener;
//use std::io::Write;
//use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:2222").unwrap();
    match listener.accept() {
        Ok((_socket, addr)) => println!("Honeypot triggered by: {:?}", addr),
        Err(e) => println!("Couldn't get client: {:?}", e),
    }

// Answer could be very long to DoS attacker, or in case of http return DoS JS
//    for stream in listener.incoming() {
//        println!("Honeypot triggered.");
//        thread::spawn(|| {
//            let mut stream = stream.unwrap();
//            stream.write(b"Hi\r\n").unwrap();
//        });

  //  }
}
