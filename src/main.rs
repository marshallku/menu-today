mod cache;
mod fetcher;
mod render;

use axum::{
    extract::{Query, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use env_logger::Env;
use fetcher::ResponseData;
use log::info;
use serde::Deserialize;
use std::{
    env,
    sync::{atomic::AtomicBool, Arc, Mutex},
    time::Instant,
};

#[derive(Clone)]
pub struct AppState {
    pub cache: Arc<Mutex<ResponseData>>,
    pub fetch_in_progress: Arc<AtomicBool>,
    pub handlebars: Arc<handlebars::Handlebars<'static>>,
}

#[derive(Deserialize)]
pub struct SVGOption {
    theme: Option<String>,
}

async fn handle_request(
    query: Query<SVGOption>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let start_time = Instant::now();
    let handlebars = state.handlebars.clone();
    info!("Clone handlebars: {:?}", start_time.elapsed());
    let data = cache::fetch_and_cache(State(state)).await.unwrap();
    info!("Fetch and cache data: {:?}", start_time.elapsed());
    let svg = render::render_svg(handlebars, &data.meals[0], query.theme.clone());
    info!("Create svg image: {:?}", start_time.elapsed());

    let mut headers = HeaderMap::new();

    headers.insert("Content-Type", "image/svg+xml".parse().unwrap());
    headers.insert("Cache-Control", "no-cache".parse().unwrap());
    headers.insert("Pragma", "no-cache".parse().unwrap());
    headers.insert("Expires", "0".parse().unwrap());

    (StatusCode::OK, headers, svg)
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let initial_data = fetcher::fetch_random_food().await.unwrap();
    let handlebars = render::create_handlebars();
    let data = AppState {
        cache: Arc::new(Mutex::new(initial_data)),
        fetch_in_progress: Arc::new(AtomicBool::new(false)),
        handlebars,
    };

    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(handle_request))
        .with_state(data);

    let bind_address = env::var("BIND_ADDRESS").unwrap_or_else(|_| String::from("127.0.0.1"));
    let addr = format!("{}:41880", bind_address);
    let listener = tokio::net::TcpListener::bind(addr.as_str()).await.unwrap();
    info!("Server running at http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}
