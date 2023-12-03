use crate::fetcher::Meal;

pub fn render_svg(meal: &Meal) -> String {
    format!(
        r#"
        <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="450px" height="200px" x="0px" y="0px" viewBox="0 0 450 200" style="enable-background:new 0 0 450 200;" xml:space="preserve">
            <text transform="matrix(1 0 0 1 9.4744 55.376)">{}</text>
        </svg>
    "#,
        meal.strMeal
    )
}
