use crate::fetcher::Meal;
use handlebars::Handlebars;

pub fn render_svg(meal: &Meal) -> String {
    let mut handlebars = Handlebars::new();
    let svg_template = r##"<svg
    width="450"
    height="200"
    viewBox="0 0 450 200"
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
        <rect width="450" height="200" fill="white" />
        <mask
            id="mask0_1_2"
            style="mask-type: alpha"
            maskUnits="userSpaceOnUse"
            x="254"
            y="0"
            width="200"
            height="200"
        >
            <rect
                x="254"
                width="200"
                height="200"
                fill="url(#paint0_linear_1_2)"
            />
        </mask>
        <g mask="url(#mask0_1_2)">
            <rect x="250" width="200" height="200" fill="url(#pattern0)" />
        </g>
    </g>
    <text
        x="12"
        y="50"
        font-size="36"
        fill="black"
        clip-path="url(#title)"
        font-weight="bold"
    >
        {{meal_name}}
    </text>
    <text
        x="12"
        y="76"
        font-size="16"
        fill="black"
        clip-path="url(#description)"
    >
        {{meal_country}} / {{meal_category}}
    </text>
    <clipPath id="title">
        <rect x="12" y="12" width="340" height="60" fill="black" />
    </clipPath>
    <clipPath id="description">
        <rect x="12" y="58" width="340" height="18" fill="black" />
    </clipPath>
    <defs>
        <pattern
            id="pattern0"
            patternContentUnits="objectBoundingBox"
            width="1"
            height="1"
        >
            <use xlink:href="#image0_1_2" transform="scale(0.00156495)" />
        </pattern>
        <linearGradient
            id="paint0_linear_1_2"
            x1="454"
            y1="91.5"
            x2="260.5"
            y2="91.5"
            gradientUnits="userSpaceOnUse"
        >
            <stop offset="0" />
            <stop offset="1" stop-opacity="0" />
        </linearGradient>
        <clipPath id="clip0_1_2">
            <rect width="450" height="200" fill="white" />
        </clipPath>
        <image
            id="image0_1_2"
            width="639"
            height="639"
            xlink:href="{{meal_thumbnail}}"
        />
    </defs>
</svg>"##;

    handlebars
        .register_template_string("svg_template", svg_template)
        .unwrap();

    let data = {
        let mut m = std::collections::BTreeMap::new();
        m.insert("meal_name", meal.strMeal.clone());
        m.insert("meal_country", meal.strArea.clone());
        m.insert("meal_category", meal.strCategory.clone());
        m.insert("meal_thumbnail", meal.strMealThumb.clone());
        m
    };

    handlebars.render("svg_template", &data).unwrap()
}
