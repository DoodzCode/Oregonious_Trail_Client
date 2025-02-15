use serde::{Deserialize, Serialize};
use std::f32::consts::E;
use std::fs::{remove_file, File};
use std::io::{self, stdin, stdout, BufReader, Write};
use std::net::{TcpStream, ToSocketAddrs};

use crate::commands;

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
        let player_profile: PlayerProfile = Client::load_player_from_file("src/config/profile_ian.json")
            .expect("Failed to load player profile");

        let host_ip: String = Client::prompt_host_ip();
        let host_port: String = Client::prompt_host_port();
        let host_addr = format!("{}:{}", host_ip, host_port);

        println!("Connecting to host: {}...", host_addr);
        let tcp_stream: TcpStream =
            TcpStream::connect(&host_addr).expect("Failed to connect to host");
        println!("Connected to host: {}", &host_addr);

        let mut client: Client = Self {
            status: ClientStatus::Waiting,
            player_profile,
            tcp_stream,
            host_addr,
        };

        client.run();
    }

    fn run(&mut self) {
        loop {
            match self.status {
                ClientStatus::Waiting => {
                    // Waiting for signal from server
                    self.handle_waiting_commands();
                    // self.status = ClientStatus::IssuingTasksOrders;
                }
                ClientStatus::IssuingTasksOrders => {
                    self.handle_issue_task_orders();
                    self.status = ClientStatus::Waiting;
                }
                ClientStatus::IssuingCaptainsOrders => {
                    self.status = ClientStatus::Inactive;
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

    fn handle_waiting_commands(&mut self) {
        self.print_hud();
        let user_input = Client::prompt_user("What would you like to do?");
        
        match user_input {
          Ok(user_input) => {
            let cmd = Client::parse_to_command(user_input);

            match cmd.0.as_str() {
              "exit" => {
                self.status = ClientStatus::Inactive;
              },
              "status" => {
                self.print_status();
              },
              "say" => {
                commands::say(cmd, &mut self.tcp_stream);
              },
              _ => {
                let msg: String = serde_json::to_string(&cmd).unwrap();
                self.tcp_stream.write(msg.as_bytes()).unwrap();
              }
            }
          },
          Err(e) => {
            println!("Error: {}", e);
          }
        }



        }

    fn handle_issue_task_orders(&mut self) {
        self.print_hud();
    }

    fn prompt_user(prompt: &str) -> Result<String, io::Error> {
        print!("{}: ", prompt);
        stdout().flush().unwrap();
        let mut response: String = String::new();
        stdin().read_line(&mut response).unwrap();
        Ok(response.trim().to_string())
    }

    fn prompt_host_ip() -> String {
      loop {
        let response = Client::prompt_user("What is the host address?");
        
        match response {
          Ok(response) => {
            if response.is_empty() {
              return String::from("127.0.0.1");
            } else {
              return response;
            }
          },
          Err(e) => {
            println!("Error: {}", e);
          }
        }
      }
    }

    fn prompt_host_port() -> String {
      loop {
        let response:Result<String, io::Error>  = Client::prompt_user("What is the host port?");
        
        match response {
          Ok(response) => {
            if response.is_empty() {
              return String::from("3000");
            } else {
              return response;
            }
          },
          Err(e) => {
            println!("Error: {}", e);
          }
        }
      }
    }

    fn load_player_from_file(filename: &str) -> serde_json::Result<PlayerProfile> {
        let file: File = File::open(filename).map_err(serde_json::Error::io)?;
        let reader: BufReader<File> = BufReader::new(file);
        let player_profile: PlayerProfile = serde_json::from_reader(reader)?;
        Ok(player_profile)
    }

    fn parse_to_command(input: String) -> (String, String) {
      match input.find(" ") {
        Some(index)  => {
          let ( cmd_name, cmd_args ) = input.split_at(index);
          (String::from(cmd_name), String::from(cmd_args))
        },
        None => (input, String::from("")),
      }
    }
  }