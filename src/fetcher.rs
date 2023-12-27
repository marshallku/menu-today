use base64::{engine::general_purpose, Engine as _};
use mime_guess::from_path;
use reqwest::{get, Error};
use serde::Deserialize;

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
    let mut response = get("https://www.themealdb.com/api/json/v1/1/random.php")
        .await?
        .json::<ResponseData>()
        .await?;
    let image_url = response.meals[0].meal_thumbnail.to_string();
    let image_response = get(&image_url).await?;
    let bytes = image_response.bytes().await?;
    let mime = from_path(&image_url).first_or_octet_stream().to_string();
    let encoded: String = general_purpose::STANDARD_NO_PAD.encode(&bytes);

    response.meals[0].meal_thumbnail = format!("data:{};base64,{}", mime, encoded);

    Ok(response.meals[0].clone())
}
