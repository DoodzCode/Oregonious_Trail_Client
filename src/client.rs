use serde::{Deserialize, Serialize};
use std::f32::consts::E;
use std::fs::{remove_file, File};
use std::io::{self, stdin, stdout, BufReader, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};

// use crate::commands;
// use crate::commands::Command;
// use crate::input_handler::InputHandler;
use crate::utils;

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
        let mut stream_copy: TcpStream = self
            .tcp_stream
            .try_clone()
            .expect("could not clone the stream");

        // launch input handler?
        // launch server handler?

        loop {
            self.print_hud();

            // ask user what they want to do?
            /*
                1. Report
                2. Give Assignment Orders ( If Available )
                3. Give Captain's Orders ( If Available )


            
            
             */


            // match self.status {
            //     ClientStatus::Waiting => {

            //         // Waiting for signal from server

            //         // let cmd: Command<String> = InputHandler::handle_waiting_commands();
            //         // self.status = ClientStatus::IssuingTasksOrders;
            //     }
            //     ClientStatus::IssuingTasksOrders => {
            //         InputHandler::handle_assignment_orders();
            //         // self.status = ClientStatus::Waiting;
            //     }
            //     ClientStatus::IssuingCaptainsOrders => {
            //         let input: String = utils::prompt_user("What are your orders?");
            //         let (cmd, args) = Client::split_input(input);

            //         let cmd_fn = self.match_command(cmd, args);

            //                             // let cmd: Result<Command<bool>, String> = InputHandler::handle_captains_orders();
            //     }
            //     ClientStatus::Inactive => {
            //         break;
            //     }
            //     _ => {}
            // }
        }
    }

    fn match_command(&self, cmd: String, args: String) {
        match cmd.as_str() {
            "order" => {
                let argv: Vec<&str> = args.split_whitespace().collect();

            },
            "report" => {},
            _ => {
                println!("Invalid command");
            }
        }
    }

    fn split_input(input: String) -> (String, String) {
        match input.find(" ") {
            Some(index) => {
                let (cmd, args) = input.split_at(index);
                (String::from(cmd), String::from(args))
            }
            None => (input, String::from(""))
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

#[derive(Debug, Serialize)]
struct Order<T> {
    directive: Directive,
    value: String,
    arguments: Vec<T>
}


impl <T>Order<T> {
    pub fn new(dir_raw: String, arguments: Vec<String>) -> Self {
        match dir_raw.as_str() {
            "proceed" => Order { directive: Directive::Proceed, arguments: vec![true] },
            "delay" => Order { directive: Directive::Proceed, arguments: vec![false]}
        }
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Debug, Serialize)]
enum Directive {
    Proceed,
}
