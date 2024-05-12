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
    let name_generator = NameGenerator::default();
    let config = AppConfig::default();
    let addr = format!("0.0.0.0:{}", config.app_port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    let app_state = Arc::new(AppState::new(config, name_generator));
    let app = make_router(app_state);
    axum::serve(listener, app).await.unwrap();
}
