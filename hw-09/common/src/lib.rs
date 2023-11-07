use std::fmt;
use serde::{Serialize, Deserialize};
use log::{info};

pub const DEFAULT_HOST: &str = "127.0.0.1";
pub const DEFAULT_PORT: &str = "11111";

#[derive(Serialize, Deserialize)]
pub enum Message {
    File(String, Vec<u8>),
    Image(String, Vec<u8>),
    Text(String),
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Message::File(name, data) => write!(f, "File"),
            Message::Image(name, data) => write!(f, "Image"),
            Message::Text(text) => write!(f, "Text"),
        }
    }
}

pub fn log_prln(message: String) {
    info!("{}", message);
    println!("{}", message);
}