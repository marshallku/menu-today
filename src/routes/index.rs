use std::time::Instant;

use axum::{
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use serde::Deserialize;
use tracing::{error, info};

use crate::{
    api::meal::{get_default_meal, get_meal},
    render::render_svg,
    utils::cache::fetch_and_cache,
    AppState,
};

#[derive(Deserialize)]
pub struct SVGOption {
    theme: Option<String>,
}

pub async fn get(query: Query<SVGOption>, State(state): State<AppState>) -> impl IntoResponse {
    let start_time = Instant::now();
    let handlebars = state.handlebars.clone();

    let mut headers = HeaderMap::new();

    headers.insert("Content-Type", "image/svg+xml".parse().unwrap());
    headers.insert("Cache-Control", "no-cache".parse().unwrap());
    headers.insert("Pragma", "no-cache".parse().unwrap());
    headers.insert("Expires", "0".parse().unwrap());

    match fetch_and_cache(get_meal, State(state)).await {
        Ok(data) => {
            let svg = render_svg(handlebars, &data, query.theme.clone());
            info!("Create svg image: {:?}", start_time.elapsed());

            (StatusCode::OK, headers, svg)
        }
        Err(e) => {
            error!("Error fetching data: {:?}", e);

            let data = get_default_meal();
            let svg = render_svg(handlebars, &data, query.theme.clone());

            (StatusCode::OK, headers, svg)
        }
    }
}
