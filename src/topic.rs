use crate::message::{Message, RegularMessage};
use serde_json;
use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct Topic {
    subscribers: Vec<TcpStream>,
}

impl Topic {
    pub fn new() -> Self {
        Topic {
            subscribers: Vec::new(),
        }
    }

    pub fn subscribe(&mut self, subscriber: TcpStream) {
        self.subscribers.push(subscriber);
    }

    pub fn publish(&mut self, message: RegularMessage) {
        let message_json = serde_json::to_string(&message)
            .unwrap_or_else(|_| String::from("Error serializing message"));
        for subscriber in &mut self.subscribers {
            writeln!(subscriber, "{}", message_json);
        }
    }
}

pub struct TopicManager {
    topics: HashMap<String, Topic>,
}

impl TopicManager {
    pub fn new() -> Self {
        TopicManager {
            topics: HashMap::new(),
        }
    }
    pub fn get_or_create_topic(&mut self, name: String) -> &mut Topic {
        self.topics.entry(name).or_insert_with(Topic::new)
    }
    pub fn get_topic(&mut self, name: String) -> Option<&mut Topic> {
        self.topics.get_mut(&name)
    }
}
