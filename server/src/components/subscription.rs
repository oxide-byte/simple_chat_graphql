use std::time::Duration;

use futures::stream::{BoxStream, StreamExt as _};
use juniper::{FieldError, graphql_subscription};
use tokio::time::interval;
use tokio_stream::wrappers::IntervalStream;
use tracing::log::{Level, log};

use crate::components::domain::ChatContext;

#[derive(Clone, Debug)]
pub struct Subscription {}

type StringStream = BoxStream<'static, Result<String, FieldError>>;

#[graphql_subscription(context = ChatContext)]
impl Subscription {
    /// Counts seconds.
    async fn last_message(
        #[graphql(context)] chat_context: &ChatContext
    ) -> StringStream {
        log!(Level::Info, "Subscription to history...");

        let mut value = 0;
        let default = "NONE".to_string();
        let message_box = chat_context.history.clone();

        let stream = IntervalStream::new(interval(Duration::from_secs(1))).map(move |_| {
            let last_message = message_box.lock().unwrap().messages.last().unwrap_or(&default).clone();

            value += 1;

            Ok(format!("{} - {}", last_message, value))
        });
        Box::pin(stream)
    }
}