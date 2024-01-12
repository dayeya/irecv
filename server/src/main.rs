use core::fmt;
use std::sync::Arc;
use std::thread;
use std::collections::HashMap;
use std::io::{Read, Write, Error};
use std::sync::mpsc::{Sender, Receiver, channel, SendError};
use std::net::{Shutdown, TcpStream, TcpListener, SocketAddr, SocketAddrV4};

const PORT: u16 = 60101;
const IP: &str = "192.168.1.218";
const BUFFER: usize = 1024;

struct Client {
    conn: Arc<TcpStream>,
    _message_count: u32,
}

impl Client {
    fn from_stream(s: Arc<TcpStream>) -> Self {
        Client {
            conn: s,
            _message_count: 0,
        }
    }

    fn peer_addr(&self) -> SocketAddr {
        self.conn.peer_addr().unwrap()
    }
}
enum Message {
    NewConnection(Arc<TcpStream>),
    ClientAborted(Arc<TcpStream>),
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
        match messages.recv().expect("The receiver is not up!") {
            Message::NewConnection(stream) => {
                let current_addr: SocketAddr = stream.peer_addr().unwrap();
                clients.insert(current_addr, Client::from_stream(stream));
            },
            Message::ClientAborted(stream) => {
                let current_addr: SocketAddr = stream.peer_addr().unwrap();
                clients.remove(&current_addr);
            },
            Message::Regular(data, sender) => {
                for (addr, client) in clients.iter_mut() {
                    if *addr != sender {
                        client.conn.as_ref().write(&data).map_err(
                            |e: Error| eprint!("ERROR: could not broadcast to {}, {e}", client.peer_addr())
                        )?;
                    }
                }
            },
        }
    }
}

fn handle_client(stream: Arc<TcpStream>, messages: Sender<Message>) -> Result<(), ()> {
    stream.as_ref().write(&"Hello, Welcome to Irecv!\n".as_bytes()).map_err(
        |e| eprintln!("Could not greet {}, {e}", stream.local_addr().unwrap())
    )?;
    messages.send(Message::NewConnection(stream.clone())).map_err(
        |e: SendError<Message>| eprintln!("ERROR: could not send a new connection, {}", e)
    )?;
    let client: Client = Client::from_stream(stream);
    let mut buff: Vec<u8> = Vec::new();
    buff.resize(BUFFER, 0);
    loop {
        match client.conn.as_ref().read(&mut buff) {
            Ok(bytes) => {
                let data: Vec<u8> = buff[..bytes].to_vec();
                messages.send(Message::Regular(data, client.peer_addr())).map_err(
                    |e: SendError<Message>| eprintln!("ERROR: could not send to receiver, {}", e)
                )?;
            },
            Err(_e) => {
                messages.send(Message::ClientAborted(client.conn.clone())).map_err(
                    |e: SendError<Message>| eprintln!("ERROR: could not info receiver about a disconnected client, {e}")
                )?;
                client.conn.shutdown(Shutdown::Both).map_err(
                    |e: Error| eprintln!("ERROR: failed to shutdown both ends of {:?}, {e}", client.peer_addr())
                )?;
            }
        }
    }
}

fn main() {
    let address: SocketAddrV4 = format!("{IP}:{PORT}").parse::<SocketAddrV4>().unwrap();
    let listener: TcpListener = TcpListener::bind(address).expect(
        &format!("ERROR: Could not start Irecv at {address}")
    );
    println!("INFO: Irecv listening for connections at {}", address);

    let (sender, receiver) = channel::<Message>();
    thread::spawn(|| server(receiver));
    
    loop  {
        match listener.accept() {
            Ok((stream, _)) => {
                let shared_stream: Arc<TcpStream> = Arc::new(stream);
                let sender: Sender<Message> = sender.clone();
                thread::spawn(|| handle_client(shared_stream, sender));
            }, 
            Err(e) => eprintln!("ERROR: failed connection, {e}")
        }
    }
}