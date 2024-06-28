use async_tungstenite::async_std::ConnectStream;
use futures::StreamExt;
use reqwest::blocking::Client as GraphClient;
use graphql_ws_client::Client as WsClient;
use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery};
use log::*;
use crate::echo_query::{MessageInput, Variables};
use graphql_ws_client::graphql::StreamingOperation;
use async_tungstenite::tungstenite::{client::IntoClientRequest, http::HeaderValue};
use async_tungstenite::WebSocketStream;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/message_query.graphql",
    response_derives = "Debug",
    normalization = "rust"
)]
struct EchoQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/count_subscription.graphql",
    response_derives = "Debug",
    normalization = "rust"
)]
struct CountSubscription;

#[async_std::main]
async fn main() {

    env_logger::init();

    let v = Variables {
        text: MessageInput {
            text: "Hello, world!".to_string(),
        }
    };

    let client = GraphClient::new();

    let response =
        post_graphql::<EchoQuery, _>(&client, "http://127.0.0.1:8080/graphql", v);

    info!("{:?}", response);

    let response_body=  response.unwrap();

    info!("{:?}", response_body);

    if let Some(errors) = response_body.errors {
        error!("there are errors:");

        for error in &errors {
            error!("{:?}", error);
        }
    }

    let response_data = response_body.data.expect("missing response data");
    println!("Response: {:?}", response_data);


    // -------------------------------------------------------------------------------------------

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
        .subscribe(StreamingOperation::<CountSubscription>::new(count_subscription::Variables, )).await
        .unwrap();

    while let Some(item) = subscription.next().await {
        println!("{item:?}");
    }
}