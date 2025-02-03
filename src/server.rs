use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

const PLAYER_AMOUNT: usize = 2;

// pub type PlayerCollection = Arc<Mutex<HashMap<SocketAddr, TcpStream>>>;
pub type PlayerCollection = HashMap<SocketAddr, TcpStream>;


pub async fn wait_for_players(port: u16) -> PlayerCollection {
    let listener = TcpListener::bind(("127.0.0.1", port))
        .await
        .expect("Could not bind to port");

    let mut players: PlayerCollection = HashMap::new();

    println!("[SERVER] Waiting for {} players to connect on port {}...", PLAYER_AMOUNT, port);

    while players.len() < PLAYER_AMOUNT {
        match listener.accept().await {
            Ok((stream, addr)) => {
                players.insert(addr, stream);
                println!("[PLAYER CONNECTED] {}/{} players connected.", &players.len(), PLAYER_AMOUNT);
            }
            Err(e) => {
                println!("[ERROR] Could not accept connection: {}", e);
            }
        }
    }

    println!("[SERVER] All players connected.");

    players

}

// pub async fn send_to_all_players(players: &PlayerCollection, message: &str) {
//     println!("Sending message to all players: {}", message);

//     let mut players_lock = players.lock().await;
//     for (_, player) in players_lock.iter_mut() {
//         let _ = player.write_all(message.as_bytes()).await;
//     }
// }
// pub async fn send_to_all_players(players: &PlayerCollection, message: &str) {
//     println!("Sending message to all players: {}", message);
//     let mut players_lock = players.lock().await;
//     for (_, player) in players_lock.iter_mut() {
//         let _ = player.write_all(message.as_bytes()).await;
//     }
// }