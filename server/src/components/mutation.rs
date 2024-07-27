use juniper::graphql_object;
use tracing::log::{Level, log};

use crate::components::domain::{ChatContext, MessageInput, MessageResponse};

#[derive(Clone, Copy, Debug)]
pub struct Mutation;

#[graphql_object]
#[graphql_object(context = ChatContext)]
impl Mutation {
    fn add_message(
        #[graphql(context)] chat_context: &ChatContext,
        message: MessageInput) -> MessageResponse {
        log!(Level::Info, "Received message: {:?}", message);
        let mut message_box = chat_context.history.lock().unwrap();
        message_box.add_message(message.text.clone());

        let echo = format!("Echo: {}", message.text);
        MessageResponse { text: Box::new(echo) }
    }
}