use crate::fetcher::Meal;
use crate::image::get_image_data_url;
use handlebars::Handlebars;

pub async fn render_svg(meal: &Meal, theme: Option<String>) -> String {
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
            @import url("https://fonts.googleapis.com/css2?family=Cabin+Sketch:wght@400;700");
            text { stroke: none; font-family: "Cabin Sketch", cursive; }
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

    let svg_width = std::cmp::max(meal.strMeal.len() * 17 + 150, 450);
    let image_x = svg_width - 200;
    let text_width = svg_width - 110;
    let text_color = match &theme {
        Some(t) if t == "dark" => "#bbb",
        Some(t) if t == "light" => "#080808",
        None => "#bbb",
        _ => "#bbb",
    };
    let background_color = match &theme {
        Some(t) if t == "dark" => "#121212",
        Some(t) if t == "light" => "#fff",
        None => "#121212",
        _ => "#121212",
    };
    let meal_thumbnail_data_url = get_image_data_url(&meal.strMealThumb).await.unwrap();
    let data = {
        let mut m = std::collections::BTreeMap::new();
        m.insert("meal_name", meal.strMeal.to_string());
        m.insert("meal_country", meal.strArea.to_string());
        m.insert("meal_category", meal.strCategory.to_string());
        m.insert("meal_thumbnail", meal_thumbnail_data_url);
        m.insert("svg_width", svg_width.to_string());
        m.insert("text_width", text_width.to_string());
        m.insert("image_x", image_x.to_string());
        m.insert("text_color", text_color.to_string());
        m.insert("background_color", background_color.to_string());
        m
    };

    handlebars.render("svg_template", &data).unwrap()
}
