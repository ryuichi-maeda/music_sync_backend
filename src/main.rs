use async_graphql::{http::GraphiQLSource, Object, Schema, SimpleObject, Subscription};
use async_graphql_axum::{GraphQL, GraphQLSubscription};
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Router,
};
use futures_util::stream::Stream;
use std::time::Duration;
use tokio::net::TcpListener;

mod query_root_resolver;

use query_root_resolver::QueryRoot;

// Mutation
struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

// Subscription
struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn interval(&self, #[graphql(default = 1)] n: i32) -> impl Stream<Item = i32> {
        let mut value = 0;
        async_stream::stream! {
            loop {
                futures_timer::Delay::new(Duration::from_secs(1)).await;
                value += n;
                yield value;
            }
        }
    }
}

async fn graphiql() -> impl IntoResponse {
    response::Html(
        GraphiQLSource::build()
            .endpoint("/graphiql")
            .subscription_endpoint("/ws")
            .finish(),
    )
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot).finish();

    let app = Router::new()
        .route(
            "/graphiql",
            get(graphiql).post_service(GraphQL::new(schema.clone())),
        )
        .route_service("/ws", GraphQLSubscription::new(schema));

    println!("GraphiQL IDE: http://localhost:8000/graphiql");

    let server = TcpListener::bind("0.0.0.0:8000").await;
    if let Err(e) = server {
        eprintln!("Error: {}", e);
        return;
    }
    axum::serve(server.unwrap(), app).await.unwrap();
}
