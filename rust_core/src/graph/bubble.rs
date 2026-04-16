use super::edge::EdgeKey;
use crate::style::BubbleStyle;
use slotmap::new_key_type;

new_key_type! {
    pub struct BubbleKey;
}

#[derive(Debug, Clone)]
pub struct Bubble {
    pub style: BubbleStyle,
    pub edges: Vec<EdgeKey>,
}

impl Bubble {
    pub fn new(style: BubbleStyle) -> Self {
        Bubble {
            style,
            edges: Vec::new(),
        }
    }

    pub fn merge(&mut self, other: &Bubble) {
        self.style = self.style.merge(&other.style);
    }

    pub fn get_pressure(&self, area: f32) -> f32 {
        let target_area = match self.style {
            BubbleStyle::Standard { size, .. } => 3000.0 * (size as f32).sqrt(),
            BubbleStyle::Player => 3000.0, // Fixed size for player
            BubbleStyle::OpenAir => 0.0,   // Open air has no target area
        };
        target_area / area.abs().max(100.0)
    }
}
