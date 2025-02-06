use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener, TcpStream};

pub type PlayerCollection = HashMap<SocketAddr, TcpStream>;

#[derive(Debug, PartialEq)]
pub enum ServerStatus {
    WaitingForHost,
    WaitingForPlayers,
    Busy,
    Idle,
    Active,
}

pub fn wait_for_players(number_of_players: u16, port: u16) -> PlayerCollection {
    let listener: TcpListener = TcpListener::bind(("0.0.0.0", port)).expect("Could not bind to port");

    let mut players: PlayerCollection = HashMap::new();

    println!(
        "[SERVER] Waiting for {} players to connect on port {}...",
        number_of_players, port
    );

    while players.len() < number_of_players as usize {
        match listener.accept() {
            Ok((stream, addr)) => {
                players.insert(addr, stream);
                println!(
                    "[PLAYER CONNECTED] {}/{} players connected.",
                    &players.len(),
                    number_of_players
                );
            }
            Err(e) => {
                println!("[ERROR] Could not accept connection: {}", e);
            }
        }
    }

    println!("[SERVER] All players connected.");

    players
}


// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::thread;
//     use std::time::Duration;

//     #[test]
//     fn test_wait_for_players() {
//         let port = 12345;
//         let number_of_players = 2;

//         thread::spawn(move || {
//             let _ = wait_for_players(number_of_players, port);
//         });

//         thread::sleep(Duration::from_millis(100));

//         for _ in 0..number_of_players {
//             let _ = TcpStream::connect(("127.0.0.1", port)).expect("Could not connect to server");
//         }
//     }

//     #[test]
//     fn test_server_status_enum() {
//         assert_eq!(ServerStatus::WaitingForHost, ServerStatus::WaitingForHost);
//         assert_eq!(
//             ServerStatus::WaitingForPlayers,
//             ServerStatus::WaitingForPlayers
//         );
//         assert_eq!(ServerStatus::Busy, ServerStatus::Busy);
//         assert_eq!(ServerStatus::Idle, ServerStatus::Idle);
//         assert_eq!(ServerStatus::Active, ServerStatus::Active);
//     }

//     #[test]
//     fn test_player_collection() {
//         let mut players: PlayerCollection = HashMap::new();
//         let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
//         let stream = TcpStream::connect(addr).expect("Could not connect to server");

//         players.insert(addr, stream);
//         assert_eq!(players.len(), 1);
//         assert!(players.contains_key(&addr));
//     }
// }
