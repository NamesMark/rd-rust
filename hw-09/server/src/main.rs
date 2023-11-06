use std::net::{TcpListener, TcpStream};
use std::fs;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use std::io::Read;
use log::{info, warn, error};

use common::Message;

use image::DynamicImage;

const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_PORT: &str = "11111";
const IMAGE_STORE: &str = "images/";
const FILE_STORE: &str = "files/";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let host = args.get(1).unwrap_or(&DEFAULT_HOST.to_string()).to_string();
    let port = args.get(2).unwrap_or(&DEFAULT_PORT.to_string()).to_string();


    start_server(&host, &port);
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    while match stream.read(&mut buffer) {
        Ok(size) => {
            let message: Message = serde_cbor::from_slice(&buffer[..size]).unwrap();
            info!("Received message: {}", String::from_utf8_lossy(&buffer[..size]));
            process_message(message);
            true
        }
        Err(e) => {
            error!("An error occurred while reading from the connection: {}", e);
            stream.shutdown(std::net::Shutdown::Both).unwrap();
            false
        }
    } {}
}


async fn start_server(host: &str, port: &str) {
    let listener = TcpListener::bind(format!("{}:{}", host, port)).unwrap();
    info!("Server listening on {}:{}", host, port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    info!("New connection: {}", stream.peer_addr().unwrap());
                    handle_client(stream)
                });
            }
            Err(e) => {
                warn!("Connection failed: {}", e);
            }
        }
    }
}

fn process_message(message: Message) {
    match message {
        Message::File(filename, data) => save_file(&filename, &data),
        Message::Image(filename, data) => save_image(&filename, &data),
        Message::Text(text) => info!("Received text message: {}", text),
    }
}

fn save_file(filename: &str, data: &[u8]) {
    fs::create_dir_all(FILE_STORE).expect("Could not create files directory");
    let path = format!("{}{}", FILE_STORE, filename);
    fs::write(path, data).expect("Could not write file");
}

fn save_image(filename: &str, data: &[u8]) {
    fs::create_dir_all(IMAGE_STORE).expect("Could not create images directory");
    save_as_png(data, filename).expect("Could not save image as PNG");
}

fn save_as_png(data: &[u8], filename: &str) -> Result<(), image::ImageError> {
    let img = image::load_from_memory(data)?;
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let file_path = format!("{}{}_{}.png", IMAGE_STORE, filename, timestamp);
    img.save_with_format(file_path, image::ImageFormat::Png)
}



fn convert_to_png(input_path: &str, output_path: &str) {
    let img = image::open(input_path).unwrap();
    img.save_with_format(output_path, image::ImageFormat::Png).unwrap();
}