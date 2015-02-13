use format_strings::{ ANSI_RESET, ANSI_YELLOW, ANSI_GREEN };
use std::io::net::ip::SocketAddr;
use std::io::net::tcp::TcpAcceptor;
use std::io::{ TcpListener, TcpStream, Acceptor, Listener, BufferedStream };
use std::sync::mpsc::channel;
use std::sync::mpsc::{ Sender, Receiver };
use std::collections::HashMap;
use std::thread::{ Thread, JoinGuard };
use std::str;
use connection::Connection;


enum Action {
    Add(SocketAddr, TcpStream),
    Remove(SocketAddr),
    Broadcast(SocketAddr, String)
}

pub fn start() {
    let listener = TcpListener::bind("0.0.0.0:6262").unwrap();
    let acceptor = listener.listen().unwrap();
    println!("Listening on 0.0.0.0:6262");
    let (sender, receiver) = channel();

    Thread::spawn(move || manage_connections(receiver));
    Thread::scoped(move || accept_connections(acceptor, sender));
}

fn accept_connections(mut acceptor: TcpAcceptor, sender: Sender<Action>) {
    loop {
        if let Ok(mut stream) = acceptor.accept() {
            if let Ok(addr) = stream.peer_name() {
                let sender = sender.clone();
                sender.send(Action::Add(addr, stream.clone())).ok();
                Thread::spawn(move || handle_connection(BufferedStream::new(stream), addr, sender));
            }
        }
    }
}

fn handle_connection(mut stream: BufferedStream<TcpStream>, addr: SocketAddr, sender: Sender<Action>) {
    stream.write_line(&*format!("----- you ({}) have connected -----", addr)).ok();
    stream.flush().ok();

    while let Ok(data) = stream.read_line() {
        sender.send(Action::Broadcast(addr, format!("{}[{}]{} {}", ANSI_GREEN, addr, ANSI_RESET, data))).ok();
    }

    sender.send(Action::Remove(addr)).ok();
}

fn manage_connections(receiver: Receiver<Action>) {
    let mut connections = HashMap::new();
    while let Ok(message) = receiver.recv() {
        match message {
            Action::Add(addr, stream)           => add_connection(&mut connections, &addr, stream),
            Action::Remove(addr)                => remove_connection(&mut connections, &addr),
            Action::Broadcast(addr, msg)        => broadcast(&mut connections, &addr, msg.as_bytes()),
        }
    }

    fn broadcast(connections:  &mut HashMap<SocketAddr, Connection>, from: &SocketAddr, msg: &[u8]) {
        println!("broadcasting msg: {}", str::from_utf8(msg).unwrap());
        for (addr, mut connection) in connections.iter_mut() {
            if *from == *addr {
                continue;
            }
            connection.stream.write(msg).ok();
            connection.stream.flush().ok();
        }
    }

    fn add_connection(connections: &mut HashMap<SocketAddr, Connection>, addr: &SocketAddr, stream: TcpStream) {
        connections.insert(*addr, Connection {
            addr: *addr,
            stream: stream,
        });
        let msg = format!("({} connections) ----- new connection from {} -----", connections.len(), addr);
        println!("{}", msg);
        broadcast(connections, addr, (msg + "\n").as_bytes());
    }

    fn remove_connection(connections: &mut HashMap<SocketAddr, Connection>, addr: &SocketAddr) {
        connections.remove(addr);
        let msg = format!("({} connections) {}----- {} is disconnected -----{}", connections.len(), ANSI_YELLOW, addr, ANSI_RESET);
        println!("{}", msg);
        broadcast(connections, addr, (msg + "\n").as_bytes());
    }
}
