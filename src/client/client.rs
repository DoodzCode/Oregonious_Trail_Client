use serde::{Deserialize, Serialize};
use std::net::TcpStream;
use crate::{structs::player::PlayerProfile, utils::prompt_user};

pub fn game_client(tcp_connection: TcpStream, player_profile: PlayerProfile) {
    let mut client_status: ClientStatus = ClientStatus::Active;

    while client_status == ClientStatus::Active {
        let response: String = prompt_user("Do you want to continue? (y/n)");
        if response == "n" {
            client_status = ClientStatus::Off;
        }
    }
}


#[derive(Debug, Deserialize, Serialize)]
struct ClientState {
    player_name: String,
    player_id: u16,
    status: ClientStatus,
}


#[derive(Debug, PartialEq, Deserialize, Serialize)]
enum ClientStatus {
    Active,
    Inactive,
    Off,
}
