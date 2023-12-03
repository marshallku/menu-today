use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Meal {
    pub idMeal: String,
    pub strMeal: String,
    pub strDrinkAlternate: Option<String>,
    pub strCategory: String,
    pub strArea: String,
    pub strInstructions: String,
    pub strMealThumb: String,
    pub strTags: String,
    pub strYoutube: String,
    pub strIngredient1: String,
    pub strIngredient2: String,
    pub strIngredient3: String,
    pub strIngredient4: String,
    pub strIngredient5: String,
    pub strIngredient6: String,
    pub strIngredient7: String,
    pub strIngredient8: String,
    pub strIngredient9: String,
    pub strIngredient10: String,
    pub strIngredient11: String,
    pub strIngredient12: String,
    pub strIngredient13: String,
    pub strIngredient14: String,
    pub strIngredient15: String,
    pub strIngredient16: String,
    pub strIngredient17: String,
    pub strIngredient18: String,
    pub strIngredient19: String,
    pub strIngredient20: String,
    pub strMeasure1: String,
    pub strMeasure2: String,
    pub strMeasure3: String,
    pub strMeasure4: String,
    pub strMeasure5: String,
    pub strMeasure6: String,
    pub strMeasure7: String,
    pub strMeasure8: String,
    pub strMeasure9: String,
    pub strMeasure10: String,
    pub strMeasure11: String,
    pub strMeasure12: String,
    pub strMeasure13: String,
    pub strMeasure14: String,
    pub strMeasure15: String,
    pub strMeasure16: String,
    pub strMeasure17: String,
    pub strMeasure18: String,
    pub strMeasure19: String,
    pub strMeasure20: String,
    pub strSource: String,
    pub strImageSource: Option<String>,
    pub strCreativeCommonsConfirmed: Option<String>,
    pub dateModified: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ResponseData {
    pub meals: Vec<Meal>,
}

pub async fn fetch_random_food() -> Result<ResponseData, Error> {
    let response = reqwest::get("https://www.themealdb.com/api/json/v1/1/random.php")
        .await?
        .json::<ResponseData>()
        .await?;
    Ok(response)
}
