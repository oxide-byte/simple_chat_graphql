use async_tungstenite::tungstenite::{client::IntoClientRequest, http::HeaderValue};
use futures::StreamExt;
use graphql_client::{GraphQLQuery, reqwest::post_graphql_blocking as post_graphql};
use graphql_ws_client::Client as WsClient;
use graphql_ws_client::graphql::StreamingOperation;
use log::*;
use reqwest::blocking::Client as GraphClient;

use crate::message_history_query::Variables;
use crate::message_modification::MessageInput;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/message_history_query.graphql",
    response_derives = "Debug",
    normalization = "rust"
)]
struct MessageHistoryQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/message_modification.graphql",
    response_derives = "Debug",
    normalization = "rust"
)]
struct MessageModification;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/message_subscription.graphql",
    response_derives = "Debug",
    normalization = "rust"
)]
struct MessageSubscription;

fn query_data(client: &GraphClient) {
    let v = Variables {};

    let response =
        post_graphql::<MessageHistoryQuery, _>(client, "http://127.0.0.1:8080/graphql", v);

    info!("{:?}", response);

    let response_body = response.unwrap();

    info!("{:?}", response_body);

    if let Some(errors) = response_body.errors {
        error!("there are errors:");

        for error in &errors {
            error!("{:?}", error);
        }
    }

    let response_data = response_body.data.expect("missing response data");
    println!("Response: {:?}", response_data);
}

fn add_data(client: &GraphClient) {
    let v = message_modification::Variables {
        message: MessageInput {
            text: "Hello, world!".to_string(),
        }
    };

    let response =
        post_graphql::<MessageModification, _>(client, "http://127.0.0.1:8080/graphql", v);

    info!("{:?}", response);

    let response_body = response.unwrap();

    info!("{:?}", response_body);

    if let Some(errors) = response_body.errors {
        error!("there are errors:");

        for error in &errors {
            error!("{:?}", error);
        }
    }

    let response_data = response_body.data.expect("missing response data");
    println!("Response: {:?}", response_data);
}

async fn subscribe_data(_client: &GraphClient) {
    let mut request = "ws://localhost:8080/subscriptions".into_client_request().unwrap();

    request.headers_mut().insert(
        "Sec-WebSocket-Protocol",
        HeaderValue::from_str("graphql-transport-ws").unwrap(),
    );

    let conn_result = async_tungstenite::async_std::connect_async(request)
        .await;

    let (connection, _) = conn_result.unwrap();

    println!("Connected");

    let mut subscription = WsClient::build(connection)
        .subscribe(StreamingOperation::<MessageSubscription>::new(message_subscription::Variables)).await
        .unwrap();

    while let Some(item) = subscription.next().await {
        println!("{item:?}");
    }
}

#[async_std::main]
async fn main() {
    env_logger::init();

    let client = GraphClient::new();

    query_data(&client);

    add_data(&client);

    query_data(&client);

    subscribe_data(&client).await;
}