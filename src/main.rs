pub mod config;
mod generator;
pub mod models;
mod routes;
pub mod schema;
use std::sync::Arc;

use crate::{config::AppConfig, generator::name_generator::NameGenerator};
use config::AppState;
use routes::make_router;

#[tokio::main]
async fn main() {
    let name_generator = NameGenerator::new();
    let config = AppConfig::default();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let app_state = Arc::new(AppState::new(config, name_generator));
    let app = make_router(app_state);
    axum::serve(listener, app).await.unwrap();
}
