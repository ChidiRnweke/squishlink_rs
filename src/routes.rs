use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::Redirect,
    routing::{get, post},
    Json, Router,
};
use rand::thread_rng;
use serde::Deserialize;

use crate::{
    config::AppState,
    errors::AppError,
    generator::{
        database::PostgresRepository,
        shorten::{OutputLink, ShortenService, Shortener},
    },
};

pub fn make_router(app_state: Arc<AppState>) -> Router {
    let state = Arc::clone(&app_state);
    Router::new()
        .route("/s", post(shorten))
        .route("/s/:short_link", get(retrieve_original_link))
        .with_state(state)
}

#[derive(Deserialize)]
struct InputLink {
    link: String,
}

async fn shorten(
    state: State<Arc<AppState>>,
    input: Json<InputLink>,
) -> Result<Json<OutputLink>, AppError> {
    let service = ShortenService::new(&state.app_config.base_url, &state.name_generator);
    let mut rng = thread_rng();
    let mut names_repo = PostgresRepository::from_config(&state.app_config.db_config)?;
    let shortened = service.shorten_name(&input.link, &mut names_repo, &mut rng)?;
    Ok(Json(shortened))
}

async fn retrieve_original_link(
    state: State<Arc<AppState>>,
    short_link: Path<String>,
) -> Result<Redirect, AppError> {
    let service = ShortenService::new(&state.app_config.base_url, &state.name_generator);
    let mut names_repo = PostgresRepository::from_config(&state.app_config.db_config)?;
    let original_maybe = service.get_original_name(&short_link, &mut names_repo);
    if let Ok(original) = original_maybe {
        Ok(Redirect::permanent(&original))
    } else {
        let missing_uri = state.app_config.base_url.clone() + "/squish/missing-url";
        Ok(Redirect::permanent(&missing_uri))
    }
}
