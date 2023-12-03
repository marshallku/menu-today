mod fetcher;
mod render;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn handle_request() -> impl Responder {
    let data = fetcher::fetch_random_food().await.unwrap();
    let svg = render::render_svg(&data.meals[0]);
    HttpResponse::Ok().content_type("image/svg+xml").body(svg)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(handle_request))
        .bind(("127.0.0.1", 41880))?
        .run()
        .await
}
