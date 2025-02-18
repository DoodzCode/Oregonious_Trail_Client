use std::io::Write;
use std::net::TcpStream;
use serde::{Deserialize, Serialize};

pub type CommandInput = (String, String);
// pub enum CommandType {
//   Client,
//   Server,
// }

#[derive(Serialize, Deserialize)]

pub enum Order {
  Say,
  Report,
  Chat,
  Travel,
  Proceed,
}



#[derive(Serialize, Deserialize)]
pub struct Command<T> {
  pub order: Order,
  pub arguments: Vec<T>,
  // function: fn(Command<T>, &mut TcpStream),
}

impl<T> Command<T> {
  pub fn new(name: &str, arguments: Vec<T>) -> Command<T> {
    match name {
      "say" => Command {
        order: Order::Say,
        arguments,
      },
      "report" => Command {
        order: Order::Report,
        arguments,
      },
      "chat" => Command {
        order: Order::Chat,
        arguments,
      },
      "travel" => Command {
        order: Order::Travel,
        arguments,
      },
      "proceed" => Command {
        order: Order::Proceed,
        arguments,
      },
      _ => Command {
        order: Order::Say,
        arguments,
      },
    }
  }
}


// type ClientCommand = fn(Command, &mut TcpStream);

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Command {
//     name: String,
//     arguments: Vec<String>,a
// }

pub fn say<T: Serialize>(cmd: Command<T>, tcp_stream: &mut TcpStream) {
    let msg = serde_json::to_string(&cmd).unwrap();
    tcp_stream.write(msg.as_bytes()).unwrap();
}

pub fn report(cmd: CommandInput, tcp_stream: &mut TcpStream) {
    // Needs access to data in the client
}

// pub fn chat(cmd: CommandInput, tcp_stream: &mut TcpStream) {
//   let order: Command<String> = Command {
//     order: "chat".to_string(),
//     arguments: vec![cmd.1],
//   };
//   let msg = serde_json::to_string(&order).unwrap();
//   tcp_stream.write(msg.as_bytes()).unwrap();
// }
// // pub fn travel() {};


pub fn proceed<T: Serialize>(cmd: Command<T>, tcp_stream: &mut TcpStream) {
  let msg: String = serde_json::to_string(&cmd).unwrap();
  tcp_stream.write(msg.as_bytes()).unwrap();
}


