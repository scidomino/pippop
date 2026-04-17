use macroquad::prelude::Color;

pub const WHITE: Color = Color::new(1.0, 1.0, 1.0, 1.0);
pub const TRANSPARENT_WHITE: Color = Color::new(1.0, 1.0, 1.0, 0.5);
pub const BLACK: Color = Color::new(0.0, 0.0, 0.0, 1.0);

// Game Colors
pub const TURQUOISE: Color = Color::new(85.0 / 255.0, 196.0 / 255.0, 200.0 / 255.0, 1.0);
pub const ROSE: Color = Color::new(212.0 / 255.0, 131.0 / 255.0, 145.0 / 255.0, 1.0);
pub const GREEN: Color = Color::new(95.0 / 255.0, 168.0 / 255.0, 69.0 / 255.0, 1.0);
pub const YELLOW: Color = Color::new(192.0 / 255.0, 199.0 / 255.0, 49.0 / 255.0, 1.0);
pub const RED: Color = Color::new(212.0 / 255.0, 31.0 / 255.0, 53.0 / 255.0, 1.0);
pub const ORANGE: Color = Color::new(236.0 / 255.0, 133.0 / 255.0, 35.0 / 255.0, 1.0);

pub const ALL_COLORS: [Color; 6] = [TURQUOISE, ROSE, GREEN, YELLOW, RED, ORANGE];

pub fn get_group(size: usize) -> Vec<Color> {
    ALL_COLORS[..size.min(ALL_COLORS.len())].to_vec()
}

pub fn random_game_color() -> Color {
    ALL_COLORS[macroquad::rand::gen_range(0, ALL_COLORS.len())]
}
