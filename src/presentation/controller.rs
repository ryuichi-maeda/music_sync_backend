use async_graphql::{http::GraphiQLSource, Object, Schema, Subscription};
use async_graphql_axum::{GraphQL, GraphQLSubscription};
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Router,
};
use futures_util::stream::Stream;
use std::time::Duration;
use sqlx::MySqlPool;

use crate::resolver::query_root_resolver::{Dependency, QueryRoot};

pub async fn create_router(dependency: Dependency) -> Router {
    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot).data(dependency).finish();

    let router = Router::new()
        .route(
            "/graphiql",
            get(graphiql).post_service(GraphQL::new(schema.clone())),
        )
        .route_service("/ws", GraphQLSubscription::new(schema));

     router
}

pub async fn graphiql() -> impl IntoResponse {
    response::Html(
        GraphiQLSource::build()
            .endpoint("/graphiql")
            .subscription_endpoint("/ws")
            .finish(),
    )
}

//TODO: resolverに移動
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