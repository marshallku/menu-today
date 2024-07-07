#[cfg(test)]
mod tests {
    use crate::api::meal::{get_default_meal, get_meal};

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
