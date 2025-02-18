use crate::commands::{Command, CommandInput};
use std::error::Error;
use std::io::{self, Write};
use std::io::{stdin, stdout};

pub struct InputHandler {}

impl InputHandler {
    pub fn handle_waiting_commands() -> Command<String> {
        let response: String = InputHandler::prompt_user("What would you like to do?").unwrap();
        let cmd_input: CommandInput = InputHandler::parse_to_command_input(response);
        InputHandler::match_command(cmd_input)
    }

    pub fn handle_assignment_orders() {
        // Issue task orders
    }

    pub fn handle_captains_orders() -> Result<Command<bool>, String> {
        // Issue captains orders
        let response: String = InputHandler::prompt_user("Would you like to proceed?").unwrap();

        match response.as_str() {
            "yes" => Ok(Command::new("proceed", vec![true])),
            "no" => Ok(Command::new("proceed", vec![false])),
            _ => Err("Invalid response".to_string()),
        }
    }

    fn match_command(cmd_input: CommandInput) -> Command<String> {
        match cmd_input.0.as_str() {
            "exit" => Command::new("exit", vec![]),
            "status" => Command::new("status", vec![]),
            "say" => Command::new("say", vec![cmd_input.1]),
            _ => Command::new("unknown", vec![cmd_input.0]),
        }
    }

    fn parse_to_command_input(input: String) -> CommandInput {
        match input.find(" ") {
            Some(index) => {
                let (cmd_name, cmd_args) = input.split_at(index);
                (String::from(cmd_name), String::from(cmd_args))
            }
            None => (input, String::from("")),
        }
    }

    pub fn prompt_host_ip() -> String {
        loop {
            let response: Result<String, io::Error> =
                InputHandler::prompt_user("What is the host address?");

            match response {
                Ok(response) => {
                    if response.is_empty() {
                        return String::from("127.0.0.1");
                    } else {
                        return response;
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }

    pub fn prompt_host_port() -> String {
        loop {
            let response: Result<String, io::Error> =
                InputHandler::prompt_user("What is the host port?");

            match response {
                Ok(response) => {
                    if response.is_empty() {
                        return String::from("3000");
                    } else {
                        return response;
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
    }

    pub fn prompt_user(prompt: &str) -> Result<String, io::Error> {
        print!("{}: ", prompt);
        stdout().flush().unwrap();
        let mut response: String = String::new();
        stdin().read_line(&mut response).unwrap();
        Ok(response.trim().to_string())
    }
}
