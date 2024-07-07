use reqwest::{get, Error};
use serde::Deserialize;
use tracing::error;

use crate::utils::encode::encode_image_from_url;

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

pub async fn get_meal() -> Result<MealData, Error> {
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
    let mut meal = match data.meals.first() {
        Some(meal) => meal.clone(),
        None => {
            error!("No meals found");
            return Ok(get_default_meal());
        }
    };

    let encoded_thumbnail = match encode_image_from_url(&meal.meal_thumbnail).await {
        Ok(encoded_thumbnail) => encoded_thumbnail,
        Err(e) => {
            error!("Error encoding image: {:?}", e);
            return Ok(get_default_meal());
        }
    };

    meal.meal_thumbnail = encoded_thumbnail;

    Ok(meal)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_random_food_not_default() {
        let meal = get_meal().await.unwrap();
        let default_meal = get_default_meal();

        assert!(meal.meal_name != default_meal.meal_name);
        assert!(meal.meal_country != default_meal.meal_country);
        assert!(meal.meal_category != default_meal.meal_category);
        assert!(meal.meal_thumbnail != default_meal.meal_thumbnail);
    }
}
