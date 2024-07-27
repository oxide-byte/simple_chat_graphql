use juniper::graphql_object;
use tracing::log::{Level, log};

use crate::components::domain::ChatContext;

#[derive(Clone, Copy, Debug)]
pub struct Query;

#[graphql_object]
#[graphql_object(context = ChatContext)]
impl Query {
    fn message_history(
        #[graphql(context)] chat_context: &ChatContext) -> Vec<String> {
        log!(Level::Info, "Message history request");
        let message_box = chat_context.history.lock().unwrap();
        message_box.messages.clone()
    }
}