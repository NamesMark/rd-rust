use std::fmt;
use serde::{Serialize, Deserialize};

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