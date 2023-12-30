mod cache;
mod encode;
mod fetcher;
mod render;

use axum::{
    body::Body,
    extract::{Query, Request, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use serde::Deserialize;
use std::{
    env,
    sync::{atomic::AtomicBool, Arc, Mutex},
    time::Instant,
};
use tower_http::trace::{self, TraceLayer};
use tracing::{error, info, Level, Span};

#[derive(Clone)]
pub struct AppState {
    pub cache: Arc<Mutex<fetcher::MealData>>,
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

    let mut headers = HeaderMap::new();

    headers.insert("Content-Type", "image/svg+xml".parse().unwrap());
    headers.insert("Cache-Control", "no-cache".parse().unwrap());
    headers.insert("Pragma", "no-cache".parse().unwrap());
    headers.insert("Expires", "0".parse().unwrap());

    match cache::fetch_and_cache(fetcher::fetch_random_food, State(state)).await {
        Ok(data) => {
            let svg = render::render_svg(handlebars, &data, query.theme.clone());
            info!("Create svg image: {:?}", start_time.elapsed());

            (StatusCode::OK, headers, svg)
        }
        Err(e) => {
            error!("Error fetching data: {:?}", e);

            let data = fetcher::get_default_meal();
            let svg = render::render_svg(handlebars, &data, query.theme.clone());

            (StatusCode::OK, headers, svg)
        }
    }
}

fn trace_layer_on_request(request: &Request<Body>, _span: &Span) {
    let user_agent = request
        .headers()
        .get("user-agent")
        .map_or("<no user-agent>", |h| {
            h.to_str().unwrap_or("<invalid utf8>")
        });

    let referer = request
        .headers()
        .get("referer")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("<no referer>");

    let ip_address = request
        .headers()
        .get("x-forwarded-for")
        .or_else(|| request.headers().get("x-real-ip"))
        .and_then(|value| value.to_str().ok())
        .unwrap_or("<no ip>");

    tracing::info!(
        "User-Agent: {:?} Referrer: {:?} IP: {:?}",
        user_agent,
        referer,
        ip_address
    )
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let initial_data = fetcher::fetch_random_food().await.unwrap();
    let handlebars = render::create_handlebars();
    let data = AppState {
        cache: Arc::new(Mutex::new(initial_data)),
        fetch_in_progress: Arc::new(AtomicBool::new(false)),
        handlebars,
    };

    let app = Router::new()
        .route("/", get(handle_request))
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
