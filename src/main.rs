pub mod config;
pub mod errors;
mod generator;
mod routes;
pub mod schema;

use std::sync::Arc;

use crate::{config::AppConfig, generator::name_generator::NameGenerator};
use config::AppState;
use routes::make_router;
use simplelog::*;
use std::fs::File;

fn setup_logger() {
    let file = File::options()
        .create(true)
        .append(true)
        .open("app.log")
        .unwrap();

    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(LevelFilter::Info, Config::default(), file),
    ])
    .unwrap();
}

#[tokio::main]
async fn main() {
    setup_logger();
    let name_generator = NameGenerator::default();
    let config = AppConfig::default();
    let addr = format!("0.0.0.0:{}", config.app_port);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    let app_state = Arc::new(AppState::new(config, name_generator));
    let app = make_router(app_state);
    log::info!("App started");
    axum::serve(listener, app).await.unwrap();
}
