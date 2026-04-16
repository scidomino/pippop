use macroquad::prelude::*;

pub struct BubbleStyle {
    pub fill_color: Color,
    pub outline_color: Color,
    pub label_color: Color,
}

pub fn get_bubble_style(size: f32, is_open_air: bool) -> BubbleStyle {
    if is_open_air {
        return BubbleStyle {
            fill_color: BLANK,
            outline_color: BLANK,
            label_color: BLANK,
        };
    }

    // Map sizes to colors similar to the Android version
    let fill_color = match size as i32 {
        1 => Color::new(0.33, 0.77, 0.78, 1.0), // Turquoise
        2 => Color::new(0.83, 0.51, 0.57, 1.0), // Rose
        3 => Color::new(0.37, 0.66, 0.27, 1.0), // Green
        4 => Color::new(0.75, 0.78, 0.19, 1.0), // Yellow
        _ => Color::new(0.83, 0.12, 0.21, 1.0), // Red/High
    };

    BubbleStyle {
        fill_color,
        outline_color: WHITE,
        label_color: WHITE,
    }
}
