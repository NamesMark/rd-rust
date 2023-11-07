use common::{Message, DEFAULT_HOST, DEFAULT_PORT, log_prln};
use std::io::{self, Write};
use std::net::TcpStream;
use std::str::FromStr;
use log::{info, warn, error};

/// Test input:
/// .image nice.png
/// .image mri.jpg
/// .file trpl.pdf
/// Crabs can walk in all directions, but mostly walk and run sideways.

#[tokio::main]
async fn main() {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    let host = args.get(1).unwrap_or(&DEFAULT_HOST.to_string()).to_string();
    let port = args.get(2).unwrap_or(&DEFAULT_PORT.to_string()).to_string();


    start_client(&host, &port).await;
}

async fn start_client(host: &str, port: &str) {
    match TcpStream::connect(format!("{}:{}", host, port)) {
        Ok(mut stream) => {
            log_prln(format!("Successfully connected to server in port {}", port));

            get_input(&mut stream);
        }
        Err(e) => {
            error!("Failed to connect: {}", e);
        }
    }
}

fn get_input(stream: &mut TcpStream) {
    println!("Please enter your message (format: .file <path>, .image <path>, <any text>):");
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_ok() {
            let trimmed = line.trim();
            let message = if trimmed.starts_with(".file ") {
                let path = trimmed[6..].trim();
                Message::File(path.to_string(), std::fs::read(path).unwrap())

            } else if trimmed.starts_with(".image ") {
                let path = trimmed[7..].trim();
                Message::Image(path.to_string(), std::fs::read(path).unwrap())

            } else {
                Message::Text(trimmed.to_string())
            };
            log_prln(format!("Sending message of type {}", message));
            let serialized_message = serde_cbor::to_vec(&message).unwrap();
            stream.write_all(&serialized_message).unwrap();
        }
    }
}