use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;

const PLAYER_AMOUNT: usize = 1;

type SharedPlayers = Arc<Mutex<HashMap<SocketAddr, Arc<Mutex<TcpStream>>>>>;
type PlayerCollection = Arc<Mutex<HashMap<SocketAddr, TcpStream>>>;


pub async fn wait_for_players(port: u16) -> PlayerCollection {
    let listener = TcpListener::bind(("127.0.0.1", port))
        .await
        .expect("Could not bind to port");

    let players: PlayerCollection = Arc::new(Mutex::new(HashMap::new()));

    println!("[SERVER] Waiting for {} players to connect on port {}...", PLAYER_AMOUNT, port);

    while players.lock().await.len() < PLAYER_AMOUNT {
        match listener.accept().await {
            Ok((stream, addr)) => {
                let mut players_lock = players.lock().await;
                players_lock.insert(addr, stream);
                println!("[PLAYER CONNECTED] {}/{} players connected.", players_lock.len(), PLAYER_AMOUNT);
            }
            Err(e) => {
                println!("[ERROR] Could not accept connection: {}", e);
            }
        }
    }

    println!("[SERVER] All players connected.");

    players

}

// Handles a single player connection
// async fn handle_client(mut stream: TcpStream, addr: SocketAddr, players: SharedPlayers) {
//     println!("[NEW PLAYER] {} connected.", addr);
//     let mut players_lock = players.lock().await;
//     players_lock.insert(addr, Arc::new(Mutex::new(stream)));

//     let mut buffer = vec![0; 1024];

//     loop {
//         match stream.read(&mut buffer).await {
//             Ok(0) => {
//                 println!("[DISCONNECT] {} disconnected.", addr);
//                 break;
//             }
//             Ok(bytes_read) => {
//                 let message = String::from_utf8_lossy(&buffer[..bytes_read]);
//                 println!("[MESSAGE] {}: {}", addr, message);
//             }
//             Err(e) => {
//                 println!("[ERROR] Reading from {}: {}", addr, e);
//                 break;
//             }
//         }
//     }

//     // Remove player from the list
//     let mut players_lock = players.lock().await;
//     players_lock.remove(&addr);
// }

// /// Sends a message to all connected players
// async fn send_to_all(players: &SharedPlayers, message: &str) {
//     let mut players_lock = players.lock().await;
//     for (_, player) in players_lock.iter() {
//         let mut player_lock = player.lock().await;
//         let _ = player_lock.write_all(message.as_bytes()).await;
//     }
// }

// /// Waits for all players to send a response before proceeding
// async fn wait_for_all_players(players: &SharedPlayers) -> HashMap<SocketAddr, String> {
//     let mut responses = HashMap::new();

//     for (addr, player) in players_lock.iter() {
//         let mut player_lock = player.lock().await;
//         let mut buffer = vec![0; 1024];
//         match player_lock.read(&mut buffer).await {
//             Ok(0) => {
//                 println!("[DISCONNECT] {} disconnected.", addr);
//             }
//             Ok(bytes_read) => {
//                 let message = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
//                 responses.insert(*addr, message);
//             }
//             Err(_) => {
//                 println!("[ERROR] Could not read from {}", addr);
//             }
//         }
//     }
//     responses
// }