// This alerts after SYN and SYN-ACK is done and closes.

use std::net::TcpListener;
extern crate syslog;

use syslog::{Facility, Formatter3164};
fn syslg() {
  let formatter = Formatter3164 {
    facility: Facility::LOG_USER,
    hostname: None,
    process: "b33rus7".into(),
    pid: 42,
  };

  match syslog::unix(formatter) {
    Err(e)         => println!("impossible to connect to syslog: {:?}", e),
    Ok(mut writer) => {
      writer.err("b33rus7 triggered").expect("could not write error message");
    }
  }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:2222").unwrap();
    match listener.accept() {
        Ok((_socket, addr)) => println!("Honeypot triggered by: {:?}", addr),
        Err(e) => println!("Couldn't get client: {:?}", e),
    }
    syslg();
}


