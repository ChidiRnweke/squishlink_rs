use std::sync::Arc;

use axum::{extract::State, routing::post, Json, Router};
use rand::thread_rng;
use serde::Deserialize;

use crate::{
    config::AppState,
    generator::{
        database::PostgresRepository,
        shorten::{OutputLink, ShortenService, Shortener},
    },
};

pub fn make_router(app_state: Arc<AppState>) -> Router {
    let state = Arc::clone(&app_state);
    Router::new()
        .route("/shorten", post(shorten))
        .with_state(state)
}

#[derive(Deserialize)]
struct InputLink {
    link: String,
}

async fn shorten(
    State(state): State<Arc<AppState>>,
    Json(input): Json<InputLink>,
) -> Result<Json<OutputLink>, String> {
    let service = ShortenService::new(&state.app_config.base_url, &state.name_generator);
    let mut rng = thread_rng();
    let mut names_repo = PostgresRepository::from_config(&state.app_config.db_config);
    let shortened = service.shorten_name(&input.link, &mut names_repo, &mut rng)?;
    Ok(Json(shortened))
}
