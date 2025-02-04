use serde::{Deserialize, Serialize};


fn client() {
  let mut client_status = ClientStatus::Active;
  


  
  let response = prompt_user("How many players?");
  let number_of_players: u16 = response.parse().expect("Failed to parse string to u16");
  

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
}