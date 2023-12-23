use base64::{engine::general_purpose, Engine as _};
use mime_guess::from_path;
use reqwest::{get, Error};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[allow(non_snake_case)]
pub struct Meal {
    pub idMeal: String,
    pub strMeal: String,
    pub strDrinkAlternate: Option<String>,
    pub strCategory: String,
    pub strArea: String,
    pub strInstructions: String,
    pub strMealThumb: String,
    pub strTags: Option<String>,
    pub strYoutube: Option<String>,
    pub strIngredient1: Option<String>,
    pub strIngredient2: Option<String>,
    pub strIngredient3: Option<String>,
    pub strIngredient4: Option<String>,
    pub strIngredient5: Option<String>,
    pub strIngredient6: Option<String>,
    pub strIngredient7: Option<String>,
    pub strIngredient8: Option<String>,
    pub strIngredient9: Option<String>,
    pub strIngredient10: Option<String>,
    pub strIngredient11: Option<String>,
    pub strIngredient12: Option<String>,
    pub strIngredient13: Option<String>,
    pub strIngredient14: Option<String>,
    pub strIngredient15: Option<String>,
    pub strIngredient16: Option<String>,
    pub strIngredient17: Option<String>,
    pub strIngredient18: Option<String>,
    pub strIngredient19: Option<String>,
    pub strIngredient20: Option<String>,
    pub strMeasure1: Option<String>,
    pub strMeasure2: Option<String>,
    pub strMeasure3: Option<String>,
    pub strMeasure4: Option<String>,
    pub strMeasure5: Option<String>,
    pub strMeasure6: Option<String>,
    pub strMeasure7: Option<String>,
    pub strMeasure8: Option<String>,
    pub strMeasure9: Option<String>,
    pub strMeasure10: Option<String>,
    pub strMeasure11: Option<String>,
    pub strMeasure12: Option<String>,
    pub strMeasure13: Option<String>,
    pub strMeasure14: Option<String>,
    pub strMeasure15: Option<String>,
    pub strMeasure16: Option<String>,
    pub strMeasure17: Option<String>,
    pub strMeasure18: Option<String>,
    pub strMeasure19: Option<String>,
    pub strMeasure20: Option<String>,
    pub strSource: Option<String>,
    pub strImageSource: Option<String>,
    pub strCreativeCommonsConfirmed: Option<String>,
    pub dateModified: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ResponseData {
    pub meals: Vec<Meal>,
}

pub async fn fetch_random_food() -> Result<ResponseData, Error> {
    let mut response = get("https://www.themealdb.com/api/json/v1/1/random.php")
        .await?
        .json::<ResponseData>()
        .await?;
    let image_url = response.meals[0].strMealThumb.to_string();
    let image_response = get(&image_url).await?;
    let bytes = image_response.bytes().await?;
    let mime = from_path(&image_url).first_or_octet_stream().to_string();
    let encoded: String = general_purpose::STANDARD_NO_PAD.encode(&bytes);

    response.meals[0].strMealThumb = format!("data:{};base64,{}", mime, encoded);

    Ok(response)
}
