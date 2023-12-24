use crate::client_handler::ClientHandler;
use crate::message::Message;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};

pub struct Topic {
    subscribers: Vec<Arc<Mutex<ClientHandler>>>,
    messages: VecDeque<Message>,
}

impl Topic {
    pub fn new() -> Self {
        Topic {
            subscribers: Vec::new(),
            messages: VecDeque::new(),
        }
    }

    pub fn subscribe(&mut self, subscriber: Arc<Mutex<ClientHandler>>) {
        self.subscribers.push(subscriber);
    }

    pub fn publish(&mut self, message: Message) {
        self.messages.push_back(message);

        for subscriber in &self.subscribers {
            //subscriber.lock().unwrap().send_message(&message);
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
    pub fn get_or_create_topic(&mut self, name: &str) -> &mut Topic {
        self.topics
            .entry(name.to_string())
            .or_insert_with(Topic::new)
    }
}
