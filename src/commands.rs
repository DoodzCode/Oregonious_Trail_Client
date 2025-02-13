use std::net::TcpStream;
use std::io::Write;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Command {
  name: String,
  arguments: Vec<String>,
}

pub fn say(cmd: (String, String), tcp_stream: &mut TcpStream) {
  let cmd  = Command {
    name: cmd.0.to_string(),
    arguments: vec![cmd.1.to_string()],
  };
  let msg = serde_json::to_string(&cmd).unwrap();

  tcp_stream.write(msg.as_bytes()).unwrap();
}