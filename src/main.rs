mod fetcher;
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
    let svg = render::render_svg(&data.meals[0], query.theme.clone());
    HttpResponse::Ok().content_type("image/svg+xml").body(svg)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server =
        HttpServer::new(|| App::new().service(handle_request)).bind(("127.0.0.1", 41880))?;

    println!("Server running at http://{}", server.addrs()[0]);

    server.run().await
}
