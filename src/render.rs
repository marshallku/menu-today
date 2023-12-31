use handlebars::Handlebars;
use std::{cmp::max, collections::HashMap, sync::Arc};

use crate::fetcher::MealData;

const SVG_WIDTH: usize = 450;
const IMAGE_WIDTH: usize = 200;
const TEXT_RIGHT_GUTTER: usize = 100;

pub fn create_handlebars() -> Arc<Handlebars<'static>> {
    let mut handlebars = Handlebars::new();
    let svg_template = r##"<svg
    width="{{svg_width}}"
    height="200"
    viewBox="0 0 {{svg_width}} 200"
    fill="none"
    xmlns="http://www.w3.org/2000/svg"
    xmlns:xlink="http://www.w3.org/1999/xlink"
>
    <defs>
        <style>
            text { stroke: none; font-family: -apple-system, BlinkMacSystemFont,
                'Apple SD Gothic Neo', 'Malgun Gothic', '맑은 고딕', arial, sans-serif }
        </style>
    </defs>
    <g clip-path="url(#clip0_1_2)">
        <rect width="{{svg_width}}" height="200" fill="{{background_color}}" />
        <mask
            id="mask0_1_2"
            style="mask-type: alpha"
            maskUnits="userSpaceOnUse"
            x="{{image_x}}"
            y="0"
            width="200"
            height="200"
        >
            <rect
                x="{{image_x}}"
                width="200"
                height="200"
                fill="url(#paint0_linear_1_2)"
            />
        </mask>
        <g mask="url(#mask0_1_2)">
            <rect x="{{image_x}}" width="200" height="200" fill="url(#pattern0)" />
        </g>
    </g>
    <text
        x="12"
        y="50"
        font-size="36"
        fill="{{text_color}}"
        clip-path="url(#title)"
        font-weight="bold"
    >
        {{meal_name}}
    </text>
    <text
        x="12"
        y="76"
        font-size="16"
        fill="{{text_color}}"
        clip-path="url(#description)"
    >
        {{meal_country}} / {{meal_category}}
    </text>
    <clipPath id="title">
        <rect x="12" y="12" width="{{text_width}}" height="60" fill="black" />
    </clipPath>
    <clipPath id="description">
        <rect x="12" y="58" width="{{text_width}}" height="18" fill="black" />
    </clipPath>
    <defs>
        <pattern
            id="pattern0"
            patternContentUnits="objectBoundingBox"
            width="1"
            height="1"
            x="{{image_x}}"
        >
            <use xlink:href="#image0_1_2" transform="scale(0.005)" />
        </pattern>
        <linearGradient
            id="paint0_linear_1_2"
            x1="{{svg_width}}"
            y1="100"
            x2="{{image_x}}"
            y2="100"
            gradientUnits="userSpaceOnUse"
        >
            <stop offset="0" />
            <stop offset="1" stop-opacity="0" />
        </linearGradient>
        <clipPath id="clip0_1_2">
            <rect width="{{svg_width}}" height="200" fill="{{background_color}}" />
        </clipPath>
        <image
            id="image0_1_2"
            width="200"
            height="200"
            xlink:href="{{meal_thumbnail}}"
        />
    </defs>
</svg>"##;

    handlebars
        .register_template_string("svg_template", svg_template)
        .unwrap();

    Arc::new(handlebars)
}

pub fn render_svg(
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
