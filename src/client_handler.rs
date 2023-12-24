use crate::message::{InitMessage, Message, RegularMessage};
use crate::topic::{Topic, TopicManager};
use serde_json::{Error, Value};
use std::fmt::Debug;
use std::io::{self, prelude::*, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;
pub enum ClientState {
    New,
    Publisher,
    Subscriber,
}

pub struct ClientHandler {
    stream: TcpStream,
    state: ClientState,
    topic_manager: Arc<Mutex<TopicManager>>,
    topic_name: String,
}

impl ClientHandler {
    pub fn new(
        stream: TcpStream,
        topic_manager: Arc<Mutex<TopicManager>>,
        topic_name: String,
    ) -> Self {
        ClientHandler {
            stream: stream,
            state: ClientState::New,
            topic_manager,
            topic_name,
        }
    }
    pub fn handle(&mut self) -> io::Result<()> {
        loop {
            let mut s = String::new();
            let mut buf_reader = BufReader::new(&mut self.stream);
            match buf_reader.read_line(&mut s) {
                Ok(_) => {
                    match Message::from_json(&s) {
                        Ok(message) => match self.state {
                            ClientState::New => match message {
                                Message::InitMessage(init_msg) => {
                                    self.state = if init_msg.method == "publish" {
                                        ClientState::Publisher
                                    } else {
                                        ClientState::Subscriber
                                    };
                                    self.handle_init_message(init_msg);
                                }
                                _ => {
                                    println!("First message must be an InitMessage");
                                }
                            },
                            ClientState::Publisher => match message {
                                Message::RegularMessage(reg_msg) => {
                                    self.handle_regular_message(reg_msg);
                                }
                                _ => {
                                    println!("Publisher can only send publish messages");
                                }
                            },
                            ClientState::Subscriber => {
                                println!("Subscribers should not send messages after initial subscription");
                            }
                        },
                        Err(e) => {
                            println!("invalid json: {}", e);
                            break Ok(()); // Breaking the loop and closing the connection
                        }
                    }
                }
                Err(e) => return Err(e),
            }
        }
    }

    fn handle_init_message(&mut self, message: InitMessage) -> io::Result<()> {
        match message.method.as_str() {
            "subscribe" => self.handle_subscribe(message),
            "publish" => self.handle_publish(message),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Unknown method")),
        }
    }

    fn handle_subscribe(&mut self, message: InitMessage) -> io::Result<()> {
        let topic_name = message.topic.clone();

        let topic = self
            .topic_manager
            .lock()
            .unwrap()
            .get_or_create_topic(message.topic)
            .subscribe(self.stream.try_clone().expect("could not clone the stream"));

        let ip = self.stream.peer_addr()?;
        println!(
            "For topic {} connected subscriber with ip {}",
            topic_name, ip
        );

        Ok(())
    }

    fn handle_publish(&mut self, message: InitMessage) -> io::Result<()> {
        let topic_name = message.topic.clone();

        let topic = self
            .topic_manager
            .lock()
            .unwrap()
            .get_or_create_topic(message.topic);

        self.topic_name = topic_name.clone();

        let ip = self.stream.peer_addr()?;
        println!(
            "For topic {} connected publisher with ip {}",
            topic_name, ip
        );

        Ok(())
    }

    fn handle_regular_message(&mut self, message: RegularMessage) -> io::Result<()> {
        let topic_name = self.topic_name.clone();
        match self.topic_manager.lock().unwrap().get_topic(topic_name) {
            Some(topic) => topic.publish(message),
            None => println!("such topic does not exit yet"),
        }
        Ok(())
    }
}
