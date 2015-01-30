use std::io::net::pipe::UnixStream;
use std::os;

pub static SOCKET_PATH: &'static str = "/tmp/rust-chat.sock";

fn read_arguments() -> (String) {
    let args = os::args();

    let username = match args.as_slice() {
        [_, ref username] => username.as_slice(),
        _                 => panic!("wrong number of arguments")
    };

    return username.to_string();
}

fn main() {
    let username = read_arguments();

    println!("username: {}", username);
    // let args = os::args();

    // let socket = Path::new(SOCKET_PATH);

    // // first argument is the username
    // let username = match args.as_slice() {
    //     [_, ref username] => username.as_slice(),
    //     _                 => panic!("wrong number of arguments")
    // }

    // // connect to socket
    // let mut stream = match UnixStream::connect(&socket) {
    //     Err(_)     => panic!("server is not running"),
    //     Ok(stream) => stream,
    // };

    // // send message
    // match stream.write_str(message) {
    //     Err(_) => panic!("couldn't send message"),
    //     Ok(_)  => {}
    // }
}
