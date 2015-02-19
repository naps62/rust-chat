#![feature(io, std_misc)]

mod format_strings;
mod chat_server;
mod connection;

fn main() {
    chat_server::start();
}
