use crate::client_handler::ClientHandler;
use crate::error::KafkaError;
use crate::topic::TopicManager;
use std::io::Error;
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Server {
    listener: TcpListener,
    topic_manager: Arc<Mutex<TopicManager>>,
}

impl Server {
    pub fn new(addr: &str) -> std::io::Result<Self> {
        let topic_manager = Arc::new(Mutex::new(TopicManager::new()));
        let listener = TcpListener::bind(addr)?;
        //let client_handlers = Vec::new();
        Ok(Server {
            listener,
            topic_manager,
            //client_handlers,
        })
    }

    pub fn run(&self) -> Result<(), Error> {
        loop {
            match self.listener.accept() {
                Ok((stream, _addr)) => {
                    let topic_manager = Arc::clone(&self.topic_manager);

                    thread::spawn(move || {
                        let mut client_handler = ClientHandler::new(
                            stream,
                            topic_manager, // Use the cloned Arc
                            String::from(""),
                        );

                        // Handle the client connection
                        let _ = client_handler.handle();
                    });
                    // thread::spawn(move || {
                    //     let mut client_handler = ClientHandler::new(
                    //         stream,
                    //         Arc::clone(&self.topic_manager),
                    //         String::from(""),
                    //     );

                    //     client_handler.handle();
                    // });
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                }
            }
        }
        Ok(())
    }
}
