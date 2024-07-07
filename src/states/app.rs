use std::sync::{atomic::AtomicBool, Arc, Mutex};

use crate::api::meal::MealData;

#[derive(Clone)]
pub struct AppState {
    pub cache: Arc<Mutex<MealData>>,
    pub fetch_in_progress: Arc<AtomicBool>,
    pub handlebars: Arc<handlebars::Handlebars<'static>>,
}
