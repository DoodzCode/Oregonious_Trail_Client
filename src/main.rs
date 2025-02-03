mod engine;
mod controllers;
mod processors;
mod structs;
mod utils;

extern crate chrono;
use engine::game_loop;

fn main() {
    game_loop();
}    