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

#[derive(Debug, PartialEq)]
enum ClientStatus {
    Waiting,
    Active,
    Idle,
    Busy,
}


#[tokio::main]
async fn main() {
    let mut POWER_ON: bool = true;

    // welcome
    println!("Welcome to the game!");
    let mut client_status = ClientStatus::Busy;


    //Establish Player Identity
    let player_profile: structs::player::PlayerProfile =
        load_player_from_file("src/config/profile_ian.json")
            .expect("Failed to load player profile");

    let mut tcp_connection: Option<TcpStream> = None; // Will eventually hold the TcpStream

    // I dont like this here, but it works for now
    let mut server_status_main = Arc::new(Mutex::new(ServerStatus::Busy));

    // Host or Join
    let response = prompt_user("Do you want to host or join? (h/j)");

    if (response == "h") {
        //Host
        let mut server_status_clone = Arc::clone(&server_status_main);
        let mut server_status_client = Arc::clone(&server_status_main);

        // If host -> ask how many player?
        let number_of_players: u16 = prompt_user("How many players?")
            .parse()
            .expect("Failed to parse string to u16");

        // game_engine started on new thread
        thread::spawn(move || game_loop(number_of_players, &server_status_clone));

        client_status = ClientStatus::Waiting;
        println!("[CLIENT] Setting up game...");
        sleep(Duration::from_secs(5));

        while *server_status_client.lock().unwrap() != ServerStatus::WaitingForHost {
            print!("...");
            sleep(Duration::from_secs(2));
        }
        println!("{:?}", server_status_client.lock().unwrap());

        println!("Game is ready, establishing connection...");

        if *server_status_client.lock().unwrap() == ServerStatus::WaitingForHost {
            println!("You are the host player.");
            tcp_connection =
                Some(TcpStream::connect("127.0.0.1:5000").expect("Failed to connect to host"));
            
        }

        client_status = ClientStatus::Waiting;
        
        {
            let server_status = server_status_client.lock().unwrap();
            match *server_status {
                ServerStatus::WaitingForPlayers => {
                    println!("Waiting for players to join...");
                    while client_status == ClientStatus::Waiting {
                        print!(".");
                        sleep(Duration::from_secs(2));
                    }

                }
                _ => {
                    
                    }
                }
            }
        } else {
            let host_addr = prompt_user("Enter Host IP");
            tcp_connection = Some(TcpStream::connect(host_addr).expect("Failed to connect to host"));
        }

        while POWER_ON {
            let response = prompt_user("Do you want to continue? (y/n)");
            if response == "n" {
                POWER_ON = false;
            }
    }
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
