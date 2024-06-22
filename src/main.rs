use dotenv;
use resolver::query_root_resolver::QueryDependency;
use sqlx::MySqlPool;
use tokio::net::TcpListener;
use std::sync::Arc;

use presentation::controller::create_router;

mod resolver;
mod domain_model;
mod domain_service;
mod application_service;
mod infrastructure;
mod presentation;

#[tokio::main]
async fn main() {

    let pool = MySqlPool::connect(dotenv::var("DATABASE_URL").unwrap().as_str())
        .await
        .unwrap();

    let dependency = QueryDependency::new(
        Arc::new(infrastructure::music_library_repository::MusicLibraryRepository::new(pool.clone())),
        Arc::new(infrastructure::user_repository::UserRepository::new(pool.clone())),
        Arc::new(infrastructure::room_repository::RoomRepository::new(pool)),
    );

    let router = create_router(dependency).await;

    println!("GraphiQL IDE: http://localhost:8000/graphiql");

    let server = TcpListener::bind("0.0.0.0:8000").await;
    if let Err(e) = server {
        eprintln!("Error: {}", e);
        return;
    }
    axum::serve(server.unwrap(), router).await.unwrap();
}
