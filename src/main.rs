mod fetcher;
mod image;
mod render;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SVGOption {
    theme: Option<String>,
}

#[get("/")]
async fn handle_request(query: web::Query<SVGOption>) -> impl Responder {
    let data = fetcher::fetch_random_food().await.unwrap();
    let svg = render::render_svg(&data.meals[0], query.theme.clone()).await;
    HttpResponse::Ok().content_type("image/svg+xml").body(svg)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bind_address = std::env::var("BIND_ADDRESS").unwrap_or_else(|_| String::from("127.0.0.1"));
    let server = HttpServer::new(|| App::new().service(handle_request))
        .bind((bind_address.as_str(), 41880))?;

    println!("Server running at http://{}", server.addrs()[0]);

    server.run().await
}
