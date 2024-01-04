use core::fmt;
use std::io::{Read, Write};
use std::sync::mpsc::{
    self,
    Sender, 
    Receiver, 
    channel
};
use std::net::{
    Shutdown,
    TcpStream,
    TcpListener,
    SocketAddrV4, Ipv4Addr
};
use std::thread;

const BUFFER: usize = 1024;
const PORT: u16 = 60101;
const IP: Ipv4Addr = Ipv4Addr::new(192, 168,1, 218);
const ADDRESS: SocketAddrV4 = SocketAddrV4::new(IP, PORT);

#[allow(dead_code)]
enum Message {
    Regular(Vec<u8>)
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // Message::ClientConnected(stream) => write!(f, "{}", stream.local_addr().unwrap()),
            // Message::ClientDisconnected(stream) => write!(f, "{}", stream.local_addr().unwrap()),
            Message::Regular(buffer) => write!(f, "{:?}", buffer),
        }
    }
} 

struct Server {
    receiver: Receiver<Message>,
    clients: Vec<Sender<Message>>,
}

fn server(messages: Receiver<Message>, clients: Vec<Sender<Message>>) -> Result<(), ()> {
    loop {
        let msg =  messages.recv().expect("The receiver is not up!");
        match msg {
            Message::Regular(msg) => {
                println!("A client sent: {:?}", msg);
                for client in clients.iter() {
                    client.send(Message::Regular(msg)).map_err(
                    |e| eprintln!("Could not send a message.")
                    );
                }
            },
        }
    }
}

fn handle_client(mut stream: TcpStream, messages: Sender<Message>) -> Result<(), ()> {
    let mut buff: Vec<u8> = Vec::with_capacity(BUFFER);
    loop {
        stream.read(&mut buff).map_err(
        |e| {
                eprintln!("ERROR: failed to read data from {:?} due to: {e}", stream.local_addr().unwrap());
                let _ = stream.shutdown(Shutdown::Both).map_err(
                    |e| println!("Failed shutdown at {:?}, {e}", stream.local_addr().unwrap())
                );
        })?;
        let msg: Message = Message::Regular(buff.clone()); 
         messages.send(msg).map_err(
            |e| eprintln!("Could not send to receiver, {}", e)
         )?;
    }
}


fn main() {
    let listener = TcpListener::bind(ADDRESS).expect(
        &format!("ERROR: Could not start Irecv at {}", ADDRESS)
    );
    println!("INFO: Irecv listening for connections at {}", ADDRESS);

    let (sender, receive) = mpsc::channel::<Message>();
    thread::spawn(|| server(receive));
    
    loop  {
        match listener.accept() {
            Ok((stream, client_addr)) => {
                println!("Client connected at {:?}", client_addr);
                let sender: Sender<Message> = sender.clone();
                thread::spawn (|| handle_client(stream, sender));
            }, 
            Err(e) => eprintln!("ERROR: failed connection, {e}")
        }
    }
}
