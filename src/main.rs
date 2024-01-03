use std::io::prelude::*;
use std::net::{
    TcpStream,
    TcpListener,
    SocketAddrV4, Ipv4Addr,
    Shutdown
};
const NULL: u8 = 0;
const BUFFER: usize = 1024;
const PORT: u16 = 60101;
const IP: Ipv4Addr = Ipv4Addr::new(172, 16, 50, 178);
const ADDRESS: SocketAddrV4 = SocketAddrV4::new(IP, PORT);

fn handle_client(mut stream: TcpStream) {
    let mut buff: [u8; BUFFER] = [u8::MIN; BUFFER];
    loop {
        match stream.read(&mut buff) {
            Ok(_) => {
                let msg: Vec<u8> = buff.into_iter()
                .filter(|byte| *byte != NULL)
                .collect::<Vec<u8>>();

                match std::str::from_utf8(&msg) {
                    Ok(data) => println!("A client wrote {} bytes: {:?}", data.len(), data),
                    Err(e) => eprintln!("Could not convert into utf-8 from buffer, {e}")
                }
            },
            Err(e) => {
                eprintln!("ERROR: failed to read data from {:?} due to: {e}", stream.local_addr().unwrap());
                let _ = stream.shutdown(Shutdown::Both).map_err(
                    |e| println!("Failed shutdown at {:?}, {e}", stream.local_addr().unwrap())
                );
                break
            }   
        }
    }
}

const PORT: u16 = 60101;
const IP: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const ADDRESS: SocketAddrV4 = SocketAddrV4::new(IP, PORT);

fn main() {
    let listener: TcpListener = TcpListener::bind(ADDRESS).expect(
    &format!("ERROR: Could not start Irecv at {}", ADDRESS)
    );
    println!("INFO: Irecv listening for connections at {}", ADDRESS);

    loop  {
        match listener.accept() {
            Ok((stream, client_addr)) => {
                println!("Client connected at {:?}", client_addr);
                handle_client(stream)
            }
            Err(e) => eprintln!("ERROR: failed connection, {e}")
        }
    }
}