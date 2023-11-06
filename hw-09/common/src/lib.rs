use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum Message {
    File(String, Vec<u8>),
    Image(String, Vec<u8>),
    Text(String),
}