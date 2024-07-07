use axum::{routing::get, Router};

use crate::AppState;

pub fn app() -> Router<AppState> {
    Router::new().route("/", get(super::index::get))
}
