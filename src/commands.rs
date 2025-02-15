use std::io::Write;
use std::net::TcpStream;
use serde::{Deserialize, Serialize};

pub type Command = (String, String);
pub enum CommandType {
  Client,
  Server,
}

type ClientCommand = fn(Command, &mut TcpStream);

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Command {
//     name: String,
//     arguments: Vec<String>,
// }

pub fn say(cmd: Command, tcp_stream: &mut TcpStream) {

    let msg = serde_json::to_string(&cmd).unwrap();
    tcp_stream.write(msg.as_bytes()).unwrap();
}

pub fn report(cmd: Command, tcp_stream: &mut TcpStream) {
    // Needs access to data in the client
}

pub fn chat(cmd: Command, tcp_stream: &mut TcpStream) {}



