mod cache;
mod fetcher;
mod render;

use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};
use env_logger::Env;
use fetcher::ResponseData;
use serde::Deserialize;
use std::{
    sync::{atomic::AtomicBool, Arc, Mutex},
    time::Instant,
};

pub struct AppState {
    pub cache: Mutex<ResponseData>,
    pub in_progress: AtomicBool,
    pub handlebars: Arc<handlebars::Handlebars<'static>>,
}

#[derive(Deserialize)]
pub struct SVGOption {
    theme: Option<String>,
}

#[get("/")]
async fn handle_request(
    query: web::Query<SVGOption>,
    state: web::Data<AppState>,
) -> impl Responder {
    let start_time = Instant::now();
    let handlebars = state.handlebars.clone();
    let data = cache::fetch_and_cache(state).await.unwrap();
    let svg = render::render_svg(handlebars, &data.meals[0], query.theme.clone()).await;

    println!(
        "Time taken for generating image: {:?}",
        start_time.elapsed()
    );

    HttpResponse::Ok()
        .content_type("image/svg+xml")
        .append_header(("Cache-Control", "no-cache"))
        .append_header(("Pragma", "no-cache"))
        .append_header(("Expires", "0"))
        .body(svg)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let initial_data = fetcher::fetch_random_food().await.unwrap();
    let handlebars = render::create_handlebars();
    let data = web::Data::new(AppState {
        cache: Mutex::new(initial_data),
        in_progress: AtomicBool::new(false),
        handlebars,
    });
    let bind_address = std::env::var("BIND_ADDRESS").unwrap_or_else(|_| String::from("127.0.0.1"));
    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(data.clone())
            .service(handle_request)
    })
    .bind((bind_address.as_str(), 41880))?;

    println!("Server running at http://{}", server.addrs()[0]);

    server.run().await
}
