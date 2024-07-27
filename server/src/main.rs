use std::{net::SocketAddr, sync::Arc};

use axum::{
    Extension,
    response::Html,
    Router, routing::{get, MethodFilter, on},
};

use juniper::RootNode;
use juniper_axum::{graphiql, playground, ws};
use juniper_axum::extract::JuniperRequest;
use juniper_axum::response::JuniperResponse;
use juniper_graphql_ws::ConnectionConfig;
use tokio::net::TcpListener;

use crate::components::domain::ChatContext;
use crate::components::mutation::Mutation;
use crate::components::query::Query;
use crate::components::subscription::Subscription;

mod components;

// type Schema = RootNode<'static, Query, EmptyMutation<ChatContext>, EmptySubscription<ChatContext>>;
pub type Schema = RootNode<'static, Query, Mutation, Subscription>;

async fn homepage() -> Html<&'static str> {
    "<html><h1>juniper_axum/simple example</h1>\
           <div>visit <a href=\"/graphiql\">GraphiQL</a></div>\
           <div>visit <a href=\"/playground\">GraphQL Playground</a></div>\
    </html>"
        .into()
}

async fn custom_graphql(
    Extension(schema): Extension<Arc<Schema>>,
    Extension(chat_context): Extension<ChatContext>,
    JuniperRequest(request): JuniperRequest,
) -> JuniperResponse {
    JuniperResponse(request.execute(&*schema, &chat_context).await)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // let schema = Schema::new(Query, EmptyMutation::<ChatContext>::new(), EmptySubscription::new());
    let schema = Schema::new(Query, Mutation, Subscription {});

    let chat_context = ChatContext::default();

    let app = Router::new()
        .route("/graphql", on(MethodFilter::GET.or(MethodFilter::POST), custom_graphql))
        .route("/subscriptions", get(ws::<Arc<Schema>>(ConnectionConfig::new(chat_context.clone()))))
        .route("/graphiql", get(graphiql("/graphql", "/subscriptions")))
        .route("/playground", get(playground("/graphql", "/subscriptions")))
        .route("/", get(homepage))
        .layer(Extension(Arc::new(schema)))
        .layer(Extension(chat_context));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(addr)
        .await
        .unwrap_or_else(|e| panic!("failed to listen on {addr}: {e}"));
    tracing::info!("listening on http://{addr}");
    axum::serve(listener, app)
        .await
        .unwrap_or_else(|e| panic!("failed to run `axum::serve`: {e}"));
}