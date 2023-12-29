use axum::extract::State;
use log::error;
use reqwest::Error;
use std::sync::atomic::Ordering;
use tokio::spawn;

use crate::{fetcher::MealData, AppState};

pub async fn fetch_and_cache<F, R>(
    fetch_fn: F,
    State(state): State<AppState>,
) -> Result<MealData, Error>
where
    F: Fn() -> R + Send + Sync + 'static,
    R: std::future::Future<Output = Result<MealData, Error>> + Send,
{
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
    }

    Ok(cached_data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, AtomicUsize};
    use std::sync::{Arc, Mutex};

    pub fn setup() -> AppState {
        let app_state: AppState = AppState {
            cache: Arc::new(Mutex::new(MealData {
                meal_name: "Initial Meal".to_string(),
                meal_country: "Initial Country".to_string(),
                meal_category: "Initial Category".to_string(),
                meal_thumbnail: "Initial Thumbnail".to_string(),
            })),
            fetch_in_progress: Arc::new(AtomicBool::new(false)),
            handlebars: Arc::new(handlebars::Handlebars::new()),
        };

        app_state
    }

    static FETCH_COUNTER: AtomicUsize = AtomicUsize::new(0);

    async fn mock_fetch_random_food() -> Result<MealData, Error> {
        FETCH_COUNTER.fetch_add(1, Ordering::SeqCst);
        // Return some mock data
        Ok(MealData {
            meal_name: "Mock Meal".to_string(),
            meal_country: "Mock Country".to_string(),
            meal_category: "Mock Category".to_string(),
            meal_thumbnail: "Mock Thumbnail".to_string(),
        })
    }

    #[tokio::test]
    async fn test_fetch_and_cache() {
        let app_state = setup();
        let result = fetch_and_cache(mock_fetch_random_food, State(app_state.clone())).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_concurrent_fetching() {
        let app_state = setup();

        FETCH_COUNTER.store(0, Ordering::SeqCst);

        // Simulate concurrent requests
        let handles: Vec<_> = (0..10)
            .map(|_| {
                let app_state_clone = app_state.clone();
                tokio::spawn(async move {
                    fetch_and_cache(mock_fetch_random_food, State(app_state_clone)).await
                })
            })
            .collect();

        // Wait for all tasks to complete
        for handle in handles {
            let result = handle.await.expect("Task panicked");
            assert!(result.is_ok());
        }

        // Check that only one fetch was initiated
        assert_eq!(
            FETCH_COUNTER.load(Ordering::SeqCst),
            1,
            "Fetch should be called exactly once"
        );
    }
}
