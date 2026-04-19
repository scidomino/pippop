use crate::graph::bubble::BubbleStyle;
use crate::graphics::colors;
use macroquad::prelude::*;

pub fn draw_bubble_label(style: &BubbleStyle, centroid: Vec2, camera: &Camera2D, font: &Font) {
    let label = match style {
        BubbleStyle::Standard { size, .. } => format!("{size}"),
        BubbleStyle::Player => "P".to_string(),
        BubbleStyle::OpenAir | BubbleStyle::Waiting { .. } => return,
    };

    let screen_pos = camera.world_to_screen(centroid);
    let scale = screen_width() / 1200.0;
    let target_pixel_size = 40.0 * scale;

    let font_size = 64; // High-res rasterization
    let font_scale = target_pixel_size / font_size as f32;

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
        screen_pos.x - text_dims.width / 2.0,
        screen_pos.y + text_dims.height / 2.0,
        text_params,
    );
}
