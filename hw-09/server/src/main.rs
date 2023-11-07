use std::time::{SystemTime, UNIX_EPOCH};
use std::path::PathBuf;

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::fs;
use tokio::signal;
use tokio::sync::broadcast;
use log::{info, warn, error};

use common::{Message, DEFAULT_HOST, DEFAULT_PORT};

use image::DynamicImage;

const IMAGE_STORE: &str = "images/";
const FILE_STORE: &str = "files/";

#[tokio::main]
async fn main() {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    let host = args.get(1).unwrap_or(&DEFAULT_HOST.to_string()).to_string();
    let port = args.get(2).unwrap_or(&DEFAULT_PORT.to_string()).to_string();


    let (shutdown_sender, _) = broadcast::channel(1);
    let server = tokio::spawn(start_server(host.clone(), port.clone(), shutdown_sender.subscribe()));

    let ctrl_c = async {
        signal::ctrl_c().await.expect("Failed to listen for ctrl_c signal");
        shutdown_sender.send(()).expect("Failed to send shutdown signal");
    };

    tokio::select! {
        _ = server => {},
        _ = ctrl_c => {},
    }

    info!("Server shutting down.");
}

async fn start_server(host: String, port: String, mut shutdown_signal: broadcast::Receiver<()>) {
    let listener = TcpListener::bind(format!("{}:{}", host, port))
        .await
        .expect("Failed to bind to port");
    info!("Server listening on {}:{}", host, port);

    loop {
        tokio::select! {
            Ok((socket, _)) = listener.accept() => {
                tokio::spawn(async move {
                    info!("New connection: {}", socket.peer_addr().unwrap());
                    handle_client(socket).await;
                });
            }
            _ = shutdown_signal.recv() => {
                info!("Shutdown signal received.");
                break;
            }
        }
    }
}

async fn handle_client(mut socket: TcpStream) {
    let mut len_bytes = [0u8; 4];
    if let Err(e) = socket.read_exact(&mut len_bytes).await {
        error!("Failed to read message length: {}", e);
        return;
    }
    let len = u32::from_be_bytes(len_bytes) as usize;
    info!("Message length received: {}", len);
    //let len = 1024;

    if len > 10 * 1024 * 1024 { 
        error!("Message length too large: {}", len);
        return;
    }

    let mut buffer = vec![0u8; len];
    info!("Buffer allocated with length: {}", buffer.len());
    match socket.read_exact(&mut buffer).await {
        Ok(_) => {
            info!("Message received, length: {}", buffer.len());
            match serde_cbor::from_slice(&buffer) {
                Ok(message) => {
                    info!("Received message: {:?}", message);
                    process_message(message).await;
                }
                Err(e) => {
                    error!("Deserialization error: {}", e);
                    error!("Raw data: {:?}", buffer);
                }
            }
        }
        Err(e) => {
            error!("Failed to read message: {}", e);
        }
    }
}

async fn process_message(message: Message) {
    match message {
        Message::File(filename, data) => save_file(&filename, &data).await,
        Message::Image(filename, data) => save_image(&filename, &data).await,
        Message::Text(text) => println!("Received the following text message: {}", text),
    }
}

async fn save_file(filename: &str, data: &[u8]) {
    info!("Identified message as a file.");
    tokio::fs::create_dir_all(FILE_STORE).await.expect("Could not create files directory");

    let mut file_path = PathBuf::from(FILE_STORE).join(filename);
    info!("Trying to save the file to {:?}", file_path);
    if fs::metadata(&file_path).await.is_ok() {
        info!("File already exists, making unique");
        file_path = make_path_unique(file_path).await;
    }
    info!("New file path: {:?}", file_path);

    tokio::fs::write(&file_path, data).await.expect("Could not write file");
    info!("Saved the file to {:?}.", file_path);
}

async fn make_path_unique(file_path: PathBuf) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let new_filename = if let Some(ext) = file_path.extension() {
        format!(
            "{}_{}.{}",
            file_path.file_stem().unwrap().to_string_lossy(),
            timestamp,
            ext.to_string_lossy()
        )
    } else {
        format!(
            "{}_{}",
            file_path.file_stem().unwrap().to_string_lossy(),
            timestamp
        )
    };

    file_path.with_file_name(new_filename)
}

async fn save_image(filename: &str, data: &[u8]) {
    info!("Identified message as an image.");
    if let Err(e) = tokio::fs::create_dir_all(IMAGE_STORE).await {
        error!("Could not create images directory: {}", e);
        return;
    }    
    match save_as_png(data, filename).await {
        Ok(_) => info!("Image saved successfully."),
        Err(e) => error!("Could not save image as PNG: {}", e),
    }
}

async fn save_as_png(data: &[u8], filename: &str) -> Result<(), image::ImageError> {
    let data = data.to_owned();
    let filename = filename.to_owned();
    tokio::task::spawn_blocking(move || {
        let img = image::load_from_memory(&data)?;
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let file_path = format!("{}{}_{}.png", IMAGE_STORE, filename, timestamp);
        info!("Saved the image as {}.", file_path);
        img.save_with_format(file_path, image::ImageFormat::Png)
    })
    .await
    .map_err(|e| {
        error!("Failed to join the blocking task: {}", e);
        image::ImageError::IoError(std::io::Error::new(std::io::ErrorKind::Other, "Failed to join the blocking task"))
    })?
}