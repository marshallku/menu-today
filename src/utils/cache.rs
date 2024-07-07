use axum::extract::State;
use reqwest::Error;
use std::sync::atomic::Ordering;
use tokio::spawn;
use tracing::error;

use crate::{api::meal::MealData, AppState};

pub async fn fetch_and_cache<F, R>(
    fetch_fn: F,
    State(state): State<AppState>,
) -> Result<MealData, Error>
where
    F: Fn() -> R + Send + Sync + 'static,
    R: std::future::Future<Output = Result<MealData, Error>> + Send,
{
    let should_fetch_data =
        state
            .fetch_in_progress
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst);

    if !should_fetch_data.is_ok() {
        let cached_data = state.cache.lock().unwrap();
        return Ok(cached_data.clone());
    }

    let cache = state.cache.lock().unwrap();
    let cached_data = cache.clone();

    drop(cache);

    spawn(async move {
        state.fetch_in_progress.store(true, Ordering::SeqCst);
        match fetch_fn().await {
            Ok(new_data) => {
                let mut cache = state.cache.lock().unwrap();
                *cache = new_data;
            }
            Err(e) => {
                error!("Error fetching data: {:?}", e);
            }
        }
        state.fetch_in_progress.store(false, Ordering::SeqCst);
    });

    Ok(cached_data)
}
