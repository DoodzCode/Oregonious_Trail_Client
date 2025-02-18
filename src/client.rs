use serde::{Deserialize, Serialize};
use std::f32::consts::E;
use std::fs::{remove_file, File};
use std::io::{self, stdin, stdout, BufReader, Write};
use std::net::{TcpStream, ToSocketAddrs};

use crate::commands;
use crate::commands::Command;
use crate::input_handler::InputHandler;

const PORT: u16 = 3000;

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerProfile {
    name: String,
    id: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
enum ClientStatus {
    Connecting,
    Waiting,
    IssuingTasksOrders,
    IssuingCaptainsOrders,
    Inactive,
}

pub struct Client {
    status: ClientStatus,
    player_profile: PlayerProfile,
    tcp_stream: TcpStream,
    host_addr: String,
}

impl Client {
    pub fn start() {
        let player_profile: PlayerProfile =
            Client::load_player_from_file("src/config/profile_ian.json")
                .expect("Failed to load player profile");

        let host_ip: String = InputHandler::prompt_host_ip();
        let host_port: String = InputHandler::prompt_host_port();
        let host_addr: String = format!("{}:{}", host_ip, host_port);

        println!("Connecting to host: {}...", host_addr);
        let tcp_stream: TcpStream =
            TcpStream::connect(&host_addr).expect("Failed to connect to host");
        println!("Connected to host: {}", &host_addr);

        let mut client: Client = Self {
            status: ClientStatus::IssuingCaptainsOrders,
            player_profile,
            tcp_stream,
            host_addr,
        };

        client.run();
    }

    fn run(&mut self) {
        loop {
            self.print_hud();
            match self.status {
                ClientStatus::Waiting => {
                    // Waiting for signal from server
                    let cmd = InputHandler::handle_waiting_commands();
                    // self.status = ClientStatus::IssuingTasksOrders;
                }
                ClientStatus::IssuingTasksOrders => {
                    InputHandler::handle_assignment_orders();
                    // self.status = ClientStatus::Waiting;
                }
                ClientStatus::IssuingCaptainsOrders => {
                    
                    let cmd: Result<Command<bool>, String> = InputHandler::handle_captains_orders();
                    match cmd {
                        Ok(cmd) => {
                            commands::proceed(cmd, &mut self.tcp_stream);
                            // self.status = ClientStatus::Waiting; //! Uncomment this line to return to waiting status
                        }
                        Err(e) => {
                            println!("Error: {}", e);
                        }
                    }
                }
                ClientStatus::Inactive => {
                    break;
                }
                _ => {}
            }
        }
    }

    fn print_hud(&self) {
        println!("");
        println!("--- HUD ---");
        println!("Player: {}", self.player_profile.name);
        println!("ID: {}", self.player_profile.id);
        println!("Status: {:?}", self.status);
        println!("");
    }

    fn print_status(&self) {
        println!("Status: {:?}", self.status);
    }

    fn load_player_from_file(filename: &str) -> serde_json::Result<PlayerProfile> {
        let file: File = File::open(filename).map_err(serde_json::Error::io)?;
        let reader: BufReader<File> = BufReader::new(file);
        let player_profile: PlayerProfile = serde_json::from_reader(reader)?;
        Ok(player_profile)
    }
}
