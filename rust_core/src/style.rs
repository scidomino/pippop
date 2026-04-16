use macroquad::prelude::*;
use crate::graphics::colors;

#[derive(Debug, Clone, Copy)]
pub enum BubbleStyle {
    Standard {
        size: i32,
        max_size: i32,
        color: Color,
    },
    Player,
    OpenAir,
}

impl BubbleStyle {
    pub fn get_target_area(&self) -> f32 {
        match self {
            BubbleStyle::Standard { size, .. } => 3000.0 * (*size as f32).sqrt(),
            BubbleStyle::Player => 3000.0, // Fixed size for player
            BubbleStyle::OpenAir => 0.0,   // Open air has no target area
        }
    }

    pub fn is_open_air(&self) -> bool {
        matches!(self, BubbleStyle::OpenAir)
    }

    pub fn merge(&self, other: &BubbleStyle) -> BubbleStyle {
        if other.is_open_air() {
            return BubbleStyle::OpenAir;
        }

        match (self, other) {
            (
                BubbleStyle::Standard { size: s1, max_size: m1, color },
                BubbleStyle::Standard { size: s2, .. },
            ) => BubbleStyle::Standard {
                size: s1 + s2,
                max_size: *m1,
                color: *color,
            },
            _ => *self,
        }
    }

    pub fn render(&self, points: &[Vec2], centroid: Vec2) {
        if self.is_open_air() || points.is_empty() {
            return;
        }

        match self {
            BubbleStyle::Standard { color, .. } => {
                // Draw Fill (Triangle Fan)
                for i in 0..points.len() {
                    let p1 = points[i];
                    let p2 = points[(i + 1) % points.len()];
                    draw_triangle(centroid, p1, p2, *color);
                }
                // Draw Outline
                for i in 0..points.len() {
                    let p1 = points[i];
                    let p2 = points[(i + 1) % points.len()];
                    draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, colors::WHITE);
                }
            }
            BubbleStyle::Player => {
                // Draw Player (Transparent white for now)
                for i in 0..points.len() {
                    let p1 = points[i];
                    let p2 = points[(i + 1) % points.len()];
                    draw_triangle(centroid, p1, p2, colors::TRANSPARENT_WHITE);
                }
                // Draw Outline
                for i in 0..points.len() {
                    let p1 = points[i];
                    let p2 = points[(i + 1) % points.len()];
                    draw_line(p1.x, p1.y, p2.x, p2.y, 2.0, colors::WHITE);
                }
            }
            BubbleStyle::OpenAir => {}
        }
    }

    pub fn render_ui(&self, screen_pos: Vec2) {
        let label = match self {
            BubbleStyle::Standard { size, .. } => format!("{}", size),
            BubbleStyle::Player => "P".to_string(),
            BubbleStyle::OpenAir => return,
        };

        let font_size = 20.0;
        let text_dims = measure_text(&label, None, font_size as u16, 1.0);
        
        draw_text(
            &label,
            screen_pos.x - text_dims.width / 2.0,
            screen_pos.y + text_dims.height / 2.0,
            font_size,
            colors::WHITE,
        );
    }
}