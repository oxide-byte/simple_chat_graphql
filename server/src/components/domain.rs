use std::sync::{Arc, Mutex};

use juniper::{Context, GraphQLInputObject, GraphQLObject};

#[derive(Clone, Default)]
pub struct ChatContext {
    pub history: Arc<Mutex<MessageBox>>,
}

impl Context for ChatContext {}

#[derive(GraphQLInputObject, Debug)]
pub struct MessageInput {
    pub text: String,
}

#[derive(GraphQLObject)]
pub struct MessageResponse {
    pub text: Box<String>,
}

#[derive(Clone, Default)]
pub struct MessageBox {
    pub messages: Vec<String>,
}

impl MessageBox {
    pub fn new() -> Self {
        MessageBox {
            messages: Vec::new()
        }
    }

    pub fn add_message(&mut self, message: String) {
        if self.messages.len() > 20 {
            self.messages.remove(0);
        }
        self.messages.push(message);
    }
}