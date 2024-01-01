#[allow(unused_imports)]
use std::net::{TcpListener, TcpStream};

#[allow(dead_code)]
fn handle_stream() {
    println!("Handling something...");
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
}
