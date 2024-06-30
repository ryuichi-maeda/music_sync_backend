use async_graphql::{http::GraphiQLSource, Schema};
use async_graphql_axum::{GraphQL, GraphQLSubscription};
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Router,
};

use crate::resolver::mutation_root_resolver::MutationRoot;
use crate::resolver::query_root_resolver::QueryRoot;
use crate::resolver::subscription_root_resolver::SubscriptionRoot;
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
