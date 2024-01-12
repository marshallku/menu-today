use reqwest::{get, Error};
use serde::Deserialize;
use tracing::error;

use crate::encode::encode_image_from_url;

#[derive(Deserialize, Debug, Clone)]
pub struct MealData {
    #[serde(rename = "strMeal")]
    pub meal_name: String,
    #[serde(rename = "strArea")]
    pub meal_country: String,
    #[serde(rename = "strCategory")]
    pub meal_category: String,
    #[serde(rename = "strMealThumb")]
    pub meal_thumbnail: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResponseData {
    pub meals: Vec<MealData>,
}

pub fn get_default_meal() -> MealData {
    MealData {
        meal_name: "Error".to_string(),
        meal_country: "500".to_string(),
        meal_category: "Internal Server Error".to_string(),
        meal_thumbnail: "".to_string(),
    }
}

pub async fn fetch_random_food() -> Result<MealData, Error> {
    let response = match get("https://www.themealdb.com/api/json/v2/1/random.php").await {
        Ok(response) => response,
        Err(e) => {
            error!("Error fetching data: {:?}", e);
            return Ok(get_default_meal());
        }
    };
    let data = match response.json::<ResponseData>().await {
        Ok(data) => data,
        Err(e) => {
            error!("Error parsing data: {:?}", e);
            return Ok(get_default_meal());
        }
    };
    let mut meal = data.meals.into_iter().next().unwrap();
    let encoded_thumbnail = encode_image_from_url(&meal.meal_thumbnail).await.unwrap();

    meal.meal_thumbnail = encoded_thumbnail;

    Ok(meal)
}
