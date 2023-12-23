use actix_web::{rt::spawn, web::Data};
use reqwest::Error;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Mutex,
};

use crate::fetcher::{fetch_random_food, ResponseData};

pub struct AppState {
    pub cache: Mutex<Option<ResponseData>>,
    pub in_progress: AtomicBool,
}

pub async fn fetch_and_cache(state: Data<AppState>) -> Result<ResponseData, Error> {
    let cache = state.cache.lock().unwrap();

    // Should the response contain cached data, ensure to fetch the data in preparation for the next request
    if let Some(data) = cache.as_ref() {
        let cached_data = data.clone();
        drop(cache);

        // Only spawn a new fetch if one isn't already in progress
        if !state.in_progress.load(Ordering::SeqCst) {
            state.in_progress.store(true, Ordering::SeqCst);
            spawn(async move {
                match fetch_random_food().await {
                    Ok(new_data) => {
                        let mut cache = state.cache.lock().unwrap();
                        *cache = Some(new_data);
                    }
                    Err(e) => {
                        eprintln!("Error fetching data: {:?}", e);
                    }
                }
                state.in_progress.store(false, Ordering::SeqCst);
            });
        }

        return Ok(cached_data);
    }
    drop(cache);

    let new_data = fetch_random_food().await?;
    *state.cache.lock().unwrap() = Some(new_data.clone());

    Ok(new_data)
}
