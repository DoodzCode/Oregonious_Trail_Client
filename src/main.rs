mod controllers;
mod engine;
mod processors;
mod server;
mod structs;
mod utils;

use engine::game_loop;
use server::ServerStatus;
use utils::{load_player_from_file, prompt_user};

use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread::{self, sleep};
use std::time::Duration;





#[tokio::main]
async fn main() {
    // welcome
    println!("Welcome to the game!");

    //Establish Player Identity
    let player_profile: structs::player::PlayerProfile = load_player_from_file("src/config/profile_ian.json")
        .expect("Failed to load player profile");
    
    // let (send, receive) = channel::<String>();

    // Host or Join
    //Host
    let mut server_status_main = Arc::new(Mutex::new(ServerStatus::Idle));
    let mut server_status_clone = Arc::clone(&server_status_main);

    // let mut server_status_client = Arc::clone(&server_status_main);
    // If host -> ask how many player?
    let number_of_players: u16 = prompt_user("How many players?")
        .parse()
        .expect("Failed to parse string to u16");

    // game_engine started on new thread
    thread::spawn(move || {
        game_loop(number_of_players, server_status_clone)
    });

    // when serverstatus = awaiting host
    // host_addr = 127.0.0.1:5000;


    // else if Join
        // host_addr = prompt_user("Enter Host IP");

    // client(host_addr, player_profile);
    sleep(Duration::from_secs(30));



    //connect to tcp host


    

    // start tcp listener, connectect host player

    // connect host player first then wait for other players
    // game_loop()

    // set host IP to localhost

    // If join -> ask for host ip
    // set host IP to given ip

    // connect client to host

    




    // while client_status == ClientStatus::Active {
    //     let response = prompt_user("Do you want to continue? (y/n)");
    //     if response == "n" {
    //         client_status = ClientStatus::Inactive;
    //     }
    // }

}
