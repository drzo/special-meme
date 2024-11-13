use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessage {
    pub user: String,
    pub message: String,
}

pub struct Rusty {
    messages: Arc<Mutex<Vec<ChatMessage>>>,
}

impl Rusty {
    pub fn new() -> Self {
        Rusty {
            messages: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn run(&self, addr: &str) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(addr).await?;
        println!("Rusty chatbot listening on: {}", addr);

        loop {
            let (mut stream, _) = listener.accept().await?;
            let messages = Arc::clone(&self.messages);

            tokio::spawn(async move {
                let mut buffer = [0; 1024];
                let n = stream.read(&mut buffer).await.unwrap();
                let request = String::from_utf8_lossy(&buffer[..n]);

                if request.starts_with("POST /api/chat") {
                    // Handle POST request
                    let body_start = request.find("\r\n\r\n").map(|i| i + 4).unwrap_or(0);
                    let body = &request[body_start..];
                    match serde_json::from_str::<ChatMessage>(body) {
                        Ok(chat_message) => {
                            let response = process_message(&chat_message).await;
                            let response_json = serde_json::to_string(&response).unwrap();
                            let response = format!(
                                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: POST, OPTIONS\r\nAccess-Control-Allow-Headers: Content-Type\r\nContent-Length: {}\r\n\r\n{}",
                                response_json.len(),
                                response_json
                            );
                            stream.write_all(response.as_bytes()).await.unwrap();
                        },
                        Err(_) => {
                            // Handle JSON parsing error
                            let error_response = format!(
                                "HTTP/1.1 400 Bad Request\r\nContent-Type: text/plain\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: POST, OPTIONS\r\nAccess-Control-Allow-Headers: Content-Type\r\nContent-Length: 22\r\n\r\nInvalid JSON payload"
                            );
                            stream.write_all(error_response.as_bytes()).await.unwrap();
                        }
                    }
                } else if request.starts_with("OPTIONS /api/chat") {
                    // Handle OPTIONS request for CORS
                    let response = "HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: POST, OPTIONS\r\nAccess-Control-Allow-Headers: Content-Type\r\nContent-Length: 0\r\n\r\n";
                    stream.write_all(response.as_bytes()).await.unwrap();
                } else {
                    // Handle other requests (e.g., GET requests)
                    let response = "HTTP/1.1 405 Method Not Allowed\r\nContent-Type: text/plain\r\nAccess-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: POST, OPTIONS\r\nAccess-Control-Allow-Headers: Content-Type\r\nContent-Length: 29\r\n\r\nMethod not allowed for this route";
                    stream.write_all(response.as_bytes()).await.unwrap();
                }
            });
        }
    }
}

async fn process_message(message: &ChatMessage) -> ChatMessage {
    ChatMessage {
        user: "Rusty".to_string(),
        message: format!("You said: {}", message.message),
    }
}
