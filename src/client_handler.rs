use serde_json::{Error, Value};
use std::io::{self, Read, Write};
use std::net::TcpStream;

pub struct ClientHandler {
    stream: TcpStream,
}

impl ClientHandler {
    pub fn new(stream: TcpStream) -> Self {
        ClientHandler { stream }
    }

    pub fn handle(&mut self) -> io::Result<()> {
        let mut buffer = Vec::new();
        self.stream.read_to_end(&mut buffer)?;

        match serde_json::from_slice::<Value>(&buffer) {
            Ok(message) => {
                if let Some(method) = message.get("method") {
                    match method.as_str() {
                        Some("subscribe") => self.handle_subscribe(message),
                        Some("publish") => self.handle_publish(message),
                        _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Unknown method")),
                    }
                } else {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "No method in message",
                    ))
                }
            }
            Err(e) => {
                self.send_error(&e.to_string())?;
                Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid JSON"))
            }
        }
    }

    fn handle_subscribe(&self, message: Value) -> io::Result<()> {
        println!("Handling subscribe: {:?}", message);
        Ok(())
    }

    fn handle_publish(&self, message: Value) -> io::Result<()> {
        println!("Handling publish: {:?}", message);
        Ok(())
    }

    fn send_error(&mut self, error_msg: &str) -> io::Result<()> {
        let error_response = format!("{{\"error\": \"{}\"}}", error_msg);
        self.stream.write_all(error_response.as_bytes())
    }
}
