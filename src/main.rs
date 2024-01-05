use core::fmt;
use std::collections::HashMap;
// use std::collections::HashMap;
use std::thread;
use std::io::{Read, Write};
// use std::sync::{Arc, Mutex};
// use std::collections::HashMap;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::net::{Shutdown, TcpStream, TcpListener, SocketAddr, SocketAddrV4};

const PORT: u16 = 60101;
const IP: &str = "192.168.1.218";
const BUFFER: usize = 1024;
enum Message {
    // NewConnection,
    // ClientAborted,
    Regular(Vec<u8>)
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // Message::NewConnection => Ok(()), 
            // Message::ClientAborted => Ok(()), 
            Message::Regular(buffer) => write!(f, "{:?}", buffer),
        }
    }
}

struct Client {
    addr: SocketAddr,
}

struct Server {
    clients: HashMap<Client, TcpStream>,
}

fn server(messages: Receiver<Message>) -> Result<(), ()> {
    loop {
        let msg: Message = messages.recv().expect("The receiver is not up!");
        match msg {
            // Message::NewConnection => {println!("A new client connected!")},
            // Message::ClientAborted => {println!("Oof, a client disconnected...")},
            Message::Regular(data) => {
                println!("A client sent: {:?}", data)
            },
        }
    }
}


fn handle_client(mut stream: TcpStream, messages: Sender<Message>) -> Result<(), ()> {
    stream.write(&"Hello, Welcome to the Irecv!\n".as_bytes()).map_err(
        |e| eprintln!("Could not greet {}, {e}", stream.local_addr().unwrap())
    )?;
    // messages.send(Message::NewConnection).map_err(
    //     |e| eprintln!("ERROR: Could not receive a new connection, {}", e)
    // )?;
    let mut buff: Vec<u8> = Vec::new();
    buff.resize(BUFFER, 0);
    loop {
        match stream.read(&mut buff) {
            Ok(n) => {
                messages.send(Message::Regular(buff[..n].to_vec())).map_err(
                    |e| eprintln!("ERROR: Could not send to receiver, {}", e)
                )?;
            },
            Err(e) => {
                eprintln!("ERROR failed to read data from {} due to: {e}", stream.local_addr().unwrap());
                if let Err(e) = stream.shutdown(Shutdown::Both) {
                    eprintln!("ERROR: Failed shutdown at {:?}, {e}", stream.local_addr().unwrap());
                }
            }
        }
    }
}


fn main() {
    let address: SocketAddrV4 = format!("{IP}:{PORT}").parse().unwrap();
    let listener: TcpListener = TcpListener::bind(address).expect(
        &format!("ERROR: Could not start Irecv at {address}")
    );
    println!("INFO: Irecv listening for connections at {}", address);

    let (sender, receive) = channel::<Message>();
    thread::spawn(|| server(receive));
    
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
