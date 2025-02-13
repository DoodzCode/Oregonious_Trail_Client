use std::net::TcpStream;
use std::io::Write;

pub fn say(cmd: (&str, Vec<&str>), tcp_stream: &mut TcpStream) {
  let msg = cmd.1.join(" ");
  let msg = format!("say {}", msg);
  tcp_stream.write(msg.as_bytes()).unwrap();

}