use core::fmt;
// use std::ops::Deref;
// use std::sync::Arc;
use std::thread;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::mpsc::{Sender, Receiver, channel};
use std::net::{Shutdown, TcpStream, TcpListener, SocketAddr, SocketAddrV4};

const PORT: u16 = 60101;
const IP: &str = "192.168.1.218";
const BUFFER: usize = 1024;

struct Client {
    conn: TcpStream,
    message_count: u32,
}

impl Client {
    fn from_stream(s: TcpStream) -> Self {
        Client {
            conn: s,
            message_count: 0,
        }
    }

    fn peer_addr(&self) -> SocketAddr {
        self.conn.peer_addr().unwrap()
    }
}

enum Message {
    NewConnection(TcpStream),
    ClientAborted(TcpStream),
    Regular(Vec<u8>, SocketAddr)
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Message::NewConnection(_) => Ok(()), 
            Message::ClientAborted(_) => Ok(()), 
            Message::Regular(buffer, _) => write!(f, "{:?}", buffer),
        }
    }
}

fn server(messages: Receiver<Message>) -> Result<(), ()> {
    let mut clients = HashMap::<SocketAddr, Client>::new(); 
    loop {
        let msg: Message = messages.recv().expect("The receiver is not up!");
        match msg {
            Message::NewConnection(stream) => {
                let current_addr: SocketAddr = stream.peer_addr().unwrap();
                clients.insert(current_addr, Client::from_stream(stream));
            },
            Message::ClientAborted(stream) => {
                let current_addr = stream.peer_addr().unwrap();
                clients.remove(&current_addr);
            },
            Message::Regular(data, sender) => {
                for (addr, client) in clients.iter_mut() {
                    if *addr != sender {
                        let _ = client.conn.write(&data).map_err(
                            |e| eprint!("ERROR: Could not broadcast to {}, {e}", client.peer_addr())
                        );
                    }
                }
            },
        }
    }
}

fn handle_client(mut stream: TcpStream, messages: Sender<Message>) -> Result<(), ()> {
    stream.write(&"Hello, Welcome to Irecv!\n".as_bytes()).map_err(
        |e| eprintln!("Could not greet {}, {e}", stream.local_addr().unwrap())
    )?;
    let mut client: Client = Client::from_stream(stream);
    messages.send(Message::NewConnection(client.conn.try_clone().unwrap())).map_err(
        |e| eprintln!("ERROR: Could not send a new connection, {}", e)
    )?;
    let mut buff: Vec<u8> = Vec::new();
    buff.resize(BUFFER, 0);
    loop {
        match client.conn.read(&mut buff) {
            Ok(n) => {
                let data = buff[..n].to_vec();
                messages.send(Message::Regular(data, client.peer_addr())).map_err(
                    |e| eprintln!("ERROR: Could not send to receiver, {}", e)
                )?;
            },
            Err(e) => {
                eprintln!("ERROR failed to read data from {} due to: {e}", client.peer_addr());
                if let Err(e) = client.conn.shutdown(Shutdown::Both) {
                    eprintln!("ERROR: Failed shutdown at {:?}, {e}", client.peer_addr());
                }
            }
        }
    }
}

fn main() {
    let address = format!("{IP}:{PORT}").parse::<SocketAddrV4>().unwrap();
    let listener: TcpListener = TcpListener::bind(address).expect(
        &format!("ERROR: Could not start Irecv at {address}")
    );
    println!("INFO: Irecv listening for connections at {}", address);

    let (sender, receiver) = channel::<Message>();
    thread::spawn(|| server(receiver));
    
    loop  {
        match listener.accept() {
            Ok((stream, client_addr)) => {
                println!("Client connected at {:?}", client_addr);
                let sender: Sender<Message> = sender.clone();
                thread::spawn(|| handle_client(stream, sender));
            }, 
            Err(e) => eprintln!("ERROR: failed connection, {e}")
        }
    }
}