use reqwest::{get, Error};
use serde::Deserialize;

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

pub async fn fetch_random_food() -> Result<MealData, Error> {
    let response = get("https://www.themealdb.com/api/json/v1/1/random.php")
        .await?
        .json::<ResponseData>()
        .await?;
    let mut meal = response.meals.into_iter().next().unwrap();
    let encoded_thumbnail = encode_image_from_url(&meal.meal_thumbnail).await?;

    meal.meal_thumbnail = encoded_thumbnail;

    Ok(meal)
}
