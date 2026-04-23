use crate::graph::bubble::BubbleStyle;
use crate::graphics::colors;
use macroquad::prelude::*;

pub fn draw_bubble_label(style: &BubbleStyle, centroid: Vec2, font: &Font) {
    let label = match style {
        BubbleStyle::Standard { size, .. } => format!("{size}"),
        BubbleStyle::Player => "P".to_string(),
        BubbleStyle::Popping { size, .. } => format!("{size}"),
        BubbleStyle::OpenAir | BubbleStyle::Waiting { .. } => return,
    };

    let font_size = 64;
    let font_scale = 0.4;

    let text_params = TextParams {
        font: Some(font),
        font_size,
        font_scale,
        color: colors::WHITE,
        ..Default::default()
    };

    let text_dims = measure_text(&label, Some(font), font_size, font_scale);

    draw_text_ex(
        &label,
        centroid.x - text_dims.width / 2.0,
        centroid.y + text_dims.height / 2.0,
        text_params,
    );
}
