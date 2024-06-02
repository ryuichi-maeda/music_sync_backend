use async_graphql::*;


// Query
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn ping(&self) -> Ping {
        Ping {
            status: "ok".to_string(),
            code: 200,
        }
    }
}

#[derive(SimpleObject)]
struct Ping {
    status: String,
    code: i32,
}