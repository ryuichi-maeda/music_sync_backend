use dotenv;
use sqlx::MySqlPool;
use tokio::net::TcpListener;
use std::sync::Arc;

use presentation::controller::create_router;
use domain_service::music_library_repository_trait::MusicLibraryRepositoryTrait;
use domain_service::user_repository_trait::UserRepositoryTrait;
use domain_service::room_repository_trait::RoomRepositoryTrait;

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

    let dependency = Dependency::new(
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

//Dependencies
pub struct Dependency {
    music_library_repository: Arc<dyn MusicLibraryRepositoryTrait>,
    user_repository: Arc<dyn UserRepositoryTrait>,
    room_repository: Arc<dyn RoomRepositoryTrait>,
}

impl Dependency {
    pub fn new(
        music_library_repository: Arc<dyn MusicLibraryRepositoryTrait>,
        user_repository: Arc<dyn UserRepositoryTrait>,
        room_repository: Arc<dyn RoomRepositoryTrait>,
    ) -> Self {
        Self {
            music_library_repository,
            user_repository,
            room_repository,
        }
    }

    pub fn get_music_library_repository(&self) -> Arc<dyn MusicLibraryRepositoryTrait> {
        self.music_library_repository.clone()
    }

    pub fn get_user_repository(&self) -> Arc<dyn UserRepositoryTrait> {
        self.user_repository.clone()
    }

    pub fn get_room_repository(&self) -> Arc<dyn RoomRepositoryTrait> {
        self.room_repository.clone()
    }
}