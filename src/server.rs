use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::client_handler::ClientHandler;
use crate::error::KafkaError;

pub struct Server {
    listener: TcpListener,
    running: Arc<AtomicBool>,
}

impl Server {
    pub fn new(addr: &str, running: Arc<AtomicBool>) -> std::io::Result<Self> {
        let listener = TcpListener::bind(addr)?;
        Ok(Server { listener, running })
    }

    pub fn run(&self) -> std::io::Result<()> {
        while self.running.load(Ordering::SeqCst) {
            match self.listener.accept() {
                Ok((stream, _addr)) => {
                    self.handle_connection(stream)?;
                }
                Err(e) => {
                    if !self.running.load(Ordering::SeqCst) {
                        break;
                    }
                    eprintln!("Failed to accept connection: {}", e);
                }
            }
        }

        Ok(())
    }

    fn handle_connection(&self, stream: TcpStream) -> std::io::Result<()> {
        println!("before handle");
        let mut handler = ClientHandler::new(stream);
        let r = handler.handle();
        if r.is_err() {
            println!("oups");
        }

        println!("Connection established");
        Ok(())
    }
}
