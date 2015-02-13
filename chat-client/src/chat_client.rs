use std::io::TcpStream;
use std::io;
use std::sync::mpsc::channel;

pub fn start() {
    let username = ask_username();

    let mut reader = io::stdin();
    let mut stream = TcpStream::connect("0.0.0.0:6262");

    let (sender, receiver) = channel();

    Thread::spawn(move || send_messages(sender));
    Thread::scoped(move || accept_messages())
}

    // loop {
    //     let mut msg = reader.read_line().ok().expect("Error reading your message");
    //     msg.pop();

    //     println!("sending: {}", msg);
    //     stream.write(msg.as_bytes());
    //     stream.flush().ok();
    // }

fn ask_username() -> String {
    print!("Choose a username: ");
    let mut reader = io::stdin();
    let mut username = reader.read_line().ok().expect("Error while reading username");
    username.pop();

    username
}
