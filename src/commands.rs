



// pub enum Command {
//     Order(String), // Collects data and sends an Order to the server via tcp
//     Say(String),
//     Report,
// }

pub struct Command {

}

impl Command {
    pub fn new(name: &str, args: &str) -> Self {
        match name {
            "order" => Command::Order(String::from(args)),
            "say" => {},
            "report" => {},
        }
    }
    fn dispatch(self) {
        match self {
            Command::Order => order(self),
            Command::Say => say(self),
            Command::Report => report(self),
        }
    }
    
    
    fn order(cmd: Command) {
        // let ( _, args ) = cmd.
        // let order = Order::new(cmd);
        // Order::dispatch(cmd)
    }
    
    fn say(cmd: Command) {
        
    }
    
    fn report(cmd: Command) {
        
    }
}