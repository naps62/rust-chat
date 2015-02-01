use std::io::net::ip::SocketAddr;
use std::io::TcpStream;

pub struct Connection {
    pub addr: SocketAddr,
    pub stream: TcpStream,
}
