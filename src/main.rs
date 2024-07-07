mod api;
mod render;
mod routes;
mod utils;

use api::meal::{get_meal, MealData};
use routes::app::app;
use utils::log::trace_layer_on_request;

use std::{
    env,
    sync::{atomic::AtomicBool, Arc, Mutex},
};
use tower_http::trace::{self, TraceLayer};
use tracing::{info, Level};

#[derive(Clone)]
pub struct AppState {
    pub cache: Arc<Mutex<MealData>>,
    pub fetch_in_progress: Arc<AtomicBool>,
    pub handlebars: Arc<handlebars::Handlebars<'static>>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let initial_data = get_meal().await.unwrap();
    let handlebars = render::create_handlebars();
    let data = AppState {
        cache: Arc::new(Mutex::new(initial_data)),
        fetch_in_progress: Arc::new(AtomicBool::new(false)),
        handlebars,
    };

    let app = app()
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO))
                .on_request(trace_layer_on_request),
        )
        .with_state(data);

    let bind_address = env::var("BIND_ADDRESS").unwrap_or_else(|_| String::from("127.0.0.1"));
    let addr = format!("{}:41880", bind_address);
    let listener = tokio::net::TcpListener::bind(addr.as_str()).await.unwrap();
    info!("Server running at http://{}", addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
