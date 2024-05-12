mod cleanup;
pub mod config;
pub mod errors;
mod generator;
mod routes;
pub mod schema;
use std::{future::IntoFuture, sync::Arc};

use crate::{
    cleanup::spawn_cleanup_task, config::AppConfig, generator::name_generator::NameGenerator,
};
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
    let router = make_router(Arc::clone(&app_state));
    // cleanup is an async function that awaits another async function.
    // If there was no input parameter we wouldn't have needed to do this.
    let cleanup = async move { spawn_cleanup_task(Arc::clone(&app_state)).await };
    let (res1, _) = tokio::join!(axum::serve(listener, router).into_future(), cleanup);
    res1.unwrap();
}
