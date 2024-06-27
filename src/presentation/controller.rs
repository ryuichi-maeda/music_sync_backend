use async_graphql::{http::GraphiQLSource, Schema, Subscription};
use async_graphql_axum::{GraphQL, GraphQLSubscription};
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Router,
};
use futures_util::stream::Stream;
use std::time::Duration;

use crate::resolver::mutation_root_resolver::MutationRoot;
use crate::resolver::query_root_resolver::QueryRoot;
use crate::Dependency;

pub async fn create_router(dependency: Dependency) -> Router {
    let schema = Schema::build(QueryRoot, MutationRoot, SubscriptionRoot)
        .data(dependency)
        .finish();

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
