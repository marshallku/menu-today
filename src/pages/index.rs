use handlebars::Handlebars;
use std::{cmp::max, collections::HashMap, sync::Arc};

use crate::api::meal::MealData;

const SVG_WIDTH: usize = 450;
const IMAGE_WIDTH: usize = 200;
const TEXT_RIGHT_GUTTER: usize = 100;

pub fn render(
    handlebars: Arc<Handlebars<'static>>,
    meal: &MealData,
    theme: Option<String>,
) -> String {
    // The SVG is 450px wide, but we want to make sure the text is always visible
    // The character is 17px wide on most fonts
    let svg_width = max(meal.meal_name.len() * 17 + (IMAGE_WIDTH - 20), SVG_WIDTH);
    let image_x = svg_width - IMAGE_WIDTH;
    let text_width = svg_width - TEXT_RIGHT_GUTTER;
    let (text_color, background_color) = match &theme {
        Some(t) if t == "dark" => ("#bbb", "#121212"),
        Some(t) if t == "light" => ("#080808", "#fff"),
        _ => ("#bbb", "#121212"),
    };

    let data = [
        ("meal_name", meal.meal_name.to_string()),
        ("meal_country", meal.meal_country.to_string()),
        ("meal_category", meal.meal_category.to_string()),
        ("meal_thumbnail", meal.meal_thumbnail.to_string()),
        ("svg_width", svg_width.to_string()),
        ("text_width", text_width.to_string()),
        ("image_x", image_x.to_string()),
        ("text_color", text_color.to_string()),
        ("background_color", background_color.to_string()),
    ]
    .iter()
    .cloned()
    .collect::<HashMap<_, _>>();

    handlebars.render("svg_template", &data).unwrap()
}
