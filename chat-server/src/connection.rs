use std::old_io::net::ip::SocketAddr;
use std::old_io::TcpStream;

pub struct Connection {
    pub addr: SocketAddr,
    pub stream: TcpStream,
}
