use std::sync::Arc;

use handlebars::Handlebars;

pub fn generate() -> Arc<Handlebars<'static>> {
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
