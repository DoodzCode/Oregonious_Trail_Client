use std::error::Error;
use std::io::{self, Write};
use std::io::{stdin, stdout};

use crate::commands::Command;
use crate::orders::Order;

type InputType = (String, String);
pub struct InputHandler {}

impl InputHandler {
    // pub fn handle_waiting_commands() -> Result<Command<T>, String> {
    //     let response: String = InputHandler::prompt_user("What would you like to do?").unwrap();
    //     let cmd_input = InputHandler::parse_to_command_input(response);
    //     match cmd_input {
    //         Ok(cmd_input) => Command::new(cmd_input),
    //         Err(e) => {
    //             println!("Error: {}", e);
    //             // InputHandler::handle_waiting_commands()
    //         }
    //     }
    // }

    pub fn handle_assignment_orders() {
        // Issue task orders
    }

    pub fn handle_captains_orders() -> Result<Option<Order, Command>, String> {
        // Issue captains orders
        let response: String = 
        InputHandler::prompt_user("Would you like to proceed or delay? ( 'order proceed' or 'order delay'").unwrap();
        // let parsed_response: Result<Option<Order>, String> = InputHandler::parse_response(response);
        
        // match parsed_response {
        //     Ok(parsed_response) => {
                
        //     }
        //     Err(e) => {
        //         Err(e)
        //     }
        // }
        InputHandler::parse_response(response)

    }

    fn parse_response(response: String) -> Result<Option<Order, Command>, String> {
        let mut cmd_name: &str = "";
        let mut cmd_args: &str = "";

        match input.find(" ") {
            Some(index) => {
                (cmd_name, cmd_args) = input.split_at(index);
            }
            None => {
                cmd_name = input.as_str();
            }
        }

        let order_attempt: Result<Order, String> = Order::new(cmd_name);

        match order_attempt {
            Ok(order) => {
                Ok(Some(order))
            }
            Err(e) => Err(e)
        }

    }

    // fn parse_to_command_input(input: String) -> Result<CommandInput, String> {
    //     let mut cmd_name: &str = "";
    //     let mut cmd_args: &str = "";
    //     // get command name
    //     // if command requires arguments, get them

    //     match input.find(" ") {
    //         Some(index) => {
    //             (cmd_name, cmd_args) = input.split_at(index);
    //             //let order = Order::new(cmd_name);
    //             // match order {
    //             //     Ok(order) => Ok((order, cmd_args.trim().to_string())),
    //             //     Err(e) => Err("Invalid command".to_string()),
    //             // }
    //         }
    //         None => {
    //             cmd_name = input.as_str();
    //             // match order {
    //             //     Ok(order) => Ok((order, "".to_string())),
    //             //     Err(e) => Err("Invalid command".to_string()),
    //             // }
    //         }
    //     }

    // }

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
