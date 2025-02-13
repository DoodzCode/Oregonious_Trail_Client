use std::net::TcpStream;
use std::io::Write;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Command {
  name: String,
  arguments: Vec<String>,
}

pub fn say(cmd: (&str, Vec<&str>), tcp_stream: &mut TcpStream) {
  let cmd  = Command {
    name: cmd.0.to_string(),
    arguments: cmd.1.iter().map(|s| s.to_string()).collect(),
  };
  let msg = serde_json::to_string(&cmd).unwrap();

  tcp_stream.write(msg.as_bytes()).unwrap();

}