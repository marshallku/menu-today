use axum::extract::State;
use log::error;
use reqwest::Error;
use std::sync::atomic::Ordering;
use tokio::spawn;

use crate::{
    fetcher::{fetch_random_food, MealData},
    AppState,
};

pub async fn fetch_and_cache(State(state): State<AppState>) -> Result<MealData, Error> {
    let cache = state.cache.lock().unwrap();
    let cached_data = cache.clone();

    drop(cache);

    // Only spawn a new fetch if one isn't already in progress
    let fetch_in_progress =
        state
            .fetch_in_progress
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst);

    if fetch_in_progress.is_ok() {
        spawn(async move {
            match fetch_random_food().await {
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
    }

    return Ok(cached_data);
}
