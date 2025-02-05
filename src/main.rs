mod controllers;
mod engine;
mod processors;
mod server;
mod structs;
mod utils;

use engine::game_loop;
use server::ServerStatus;
use utils::{load_player_from_file, prompt_user};

use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread::{self, sleep};
use std::time::Duration;




#[tokio::main]
async fn main() {
    // welcome
    println!("Welcome to the game!");

    let mut POWER_ON: bool = true;

    //Establish Player Identity
    let player_profile: structs::player::PlayerProfile = load_player_from_file("src/config/profile_ian.json")
        .expect("Failed to load player profile");

    let mut player_connection: Option<TcpStream> = None;
    
    // let (send, receive) = channel::<String>();

    // Host or Join
    //Host
    let mut server_status_main = Arc::new(Mutex::new(ServerStatus::Busy));
    let mut server_status_clone = Arc::clone(&server_status_main);

    // let mut server_status_client = Arc::clone(&server_status_main);
    // If host -> ask how many player?
    let number_of_players: u16 = prompt_user("How many players?")
        .parse()
        .expect("Failed to parse string to u16");

    // game_engine started on new thread
    thread::spawn(move || {
        game_loop(number_of_players, &server_status_clone)
    });

    sleep(Duration::from_secs(5));

    println!("[CLIENT] Setting up game...");
    while *server_status_main.lock().unwrap() != ServerStatus::WaitingForHost {
        sleep(Duration::from_secs(2));
    }
    println!("{:?}", server_status_main.lock().unwrap());

    println!("Game is ready, establishing connection...");

    if *server_status_main.lock().unwrap() == ServerStatus::WaitingForHost {
        println!("You are the host player.");
        player_connection = Some(TcpStream::connect("127.0.0.1:5000")
            .expect("Failed to connect to host"));

    }

    match *server_status_main.lock().unwrap() {
        _ => {
            while POWER_ON {
                let response = prompt_user("Do you want to continue? (y/n)");
                if response == "n" {
                    POWER_ON = false;
                }
            }
        }
    }

    println!("");
    
    player_connection.unwrap().write_all("Test Message".as_bytes())
        .expect("Failed to write to stream");

    sleep(Duration::from_secs(30));
}


// when serverstatus = awaiting host
// host_addr = 127.0.0.1:5000;


// else if Join
    // host_addr = prompt_user("Enter Host IP");

// client(host_addr, player_profile);

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

