use std::net::{
    TcpStream,
    TcpListener,
    SocketAddrV4, Ipv4Addr
};

#[allow(dead_code)]
fn handle_client(_stream: TcpStream) {
    // ...
}

const PORT: u16 = 60101;
const IP: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
const ADDRESS: SocketAddrV4 = SocketAddrV4::new(IP, PORT);

fn main() {
    let listener: TcpListener = TcpListener::bind(ADDRESS).unwrap();
    if let Ok(addr) = listener.local_addr() {
        println!("Irecv listening for connections at {}", addr);
    } else {
        panic!()
    }
}