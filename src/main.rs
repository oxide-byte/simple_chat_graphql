
use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema, Object};
use async_graphql_axum::GraphQL;
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Router,
};

use tokio::net::TcpListener;

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

struct Query;

#[Object]
impl Query {
    /// Returns the sum of a and b
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .finish();

    let app = Router::new().route("/", get(graphiql).post_service(GraphQL::new(schema)));

    axum::serve(TcpListener::bind("127.0.0.1:8000").await.unwrap(), app)
        .await
        .unwrap();
}