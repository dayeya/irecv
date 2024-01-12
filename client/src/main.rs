// use core::fmt;
// use std::sync::Arc;
// use std::thread;
// use std::collections::HashMap;
// use std::io::{Read, Write, Error};
use std::net::{TcpStream, SocketAddrV4, Ipv4Addr};
use std::io::{self, stdin, Write};

const PREFIX: &str = "";
const SERVER_ADDRESS: SocketAddrV4 = SocketAddrV4::new(
   Ipv4Addr::new(192, 168, 1, 218) , 60101 as u16
);

fn main() {
    let mut buffer: String = PREFIX.to_string();
    let mut conn: TcpStream = if let Ok(stream) = TcpStream::connect(SERVER_ADDRESS) {
        stream
    } else {
        panic!("You cant connect to Irecv, it is not listening...");
    };
    println!("Successfuly connected to Irecv");

    // Start main loop.
    loop {
        print!("Send >> ");
        io::stdout().flush().unwrap();
        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                let _ = conn.write(buffer.as_bytes()).map_err(
                    |error| eprintln!("Could not send msg to Irecv, {error}")
                );
                buffer = PREFIX.to_string();
            },
            Err(err) => {
                eprintln!("Failed to read your stupid input, {err}");
            }
        }
    }
}
