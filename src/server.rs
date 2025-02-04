use std::collections::HashMap;
use std::net::{TcpListener, TcpStream, SocketAddr};
// use tokio::net::{TcpListener, TcpStream};


pub type PlayerCollection = HashMap<SocketAddr, TcpStream>;

#[derive(Debug)]
pub enum ServerStatus {
    WaitingForHost,
    Busy,
    Idle,
}

pub fn wait_for_players(number_of_players: u16, port: u16) -> PlayerCollection {
    let listener = TcpListener::bind(("0.0.0.0", port))
        .expect("Could not bind to port");

    let mut players: PlayerCollection = HashMap::new();

    println!("[SERVER] Waiting for {} players to connect on port {}...", number_of_players, port);

    while players.len() < number_of_players as usize{
        match listener.accept() {
            Ok((stream, addr)) => {
                players.insert(addr, stream);
                println!("[PLAYER CONNECTED] {}/{} players connected.", &players.len(), number_of_players);
            }
            Err(e) => {
                println!("[ERROR] Could not accept connection: {}", e);
            }
        }
    }

    println!("[SERVER] All players connected.");

    players

}
