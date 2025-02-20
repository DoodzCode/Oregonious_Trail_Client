use serde::{Deserialize, Serialize};


enum DirectiveType {
  Navigation,
  Assignment,
}

#[derive(Serialize, Deserialize)]
pub enum Directive {
  Proceed(bool),
}

#[derive(Serialize, Deserialize)]
pub struct Order {
  directive: Directive,
  assignment: u16,
}


impl Order {
    pub fn new(name: &str) -> Result<Order, String> {
        match name {
            "proceed" => Ok(Order::create(Directive::Proceed(true))),
            "delay" => Ok(Order::create(Directive::Proceed(false))),
            _ => Err(String::from("No such order exists"))
        }
    }

    fn create(directive: Directive) -> Self {
      Order {
        directive,
        assignment: 0,
      }
    }

    pub fn serialize<T>(&self) -> String {
      serde_json::to_string(&self).unwrap()
    }

    // pub fn get_type(&self) -> OrderType {
    //   match self {
    //     Order::Proceed(_) => DirectiveType::Navigation,
    //   }
    // }
}