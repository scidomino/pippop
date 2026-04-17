use super::edge::EdgeKey;
use macroquad::prelude::Color;
use slotmap::new_key_type;

#[derive(Debug, Clone, Copy, PartialEq)]
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
    pub fn merge(&self, other: &BubbleStyle) -> BubbleStyle {
        if *self == BubbleStyle::OpenAir || *other == BubbleStyle::OpenAir {
            return BubbleStyle::OpenAir;
        }

        match (self, other) {
            (
                BubbleStyle::Standard {
                    size: s1,
                    max_size: m1,
                    color,
                },
                BubbleStyle::Standard { size: s2, .. },
            ) => BubbleStyle::Standard {
                size: s1 + s2,
                max_size: *m1,
                color: *color,
            },
            _ => unreachable!("merge should only be called with Standard or OpenAir styles"),
        }
    }
}

new_key_type! {
    pub struct BubbleKey;
}

#[derive(Debug, Clone)]
pub struct Bubble {
    pub style: BubbleStyle,
    // counter clockwise list of the trailing edges that (along with their leading edge twins) form the boundary of this bubble
    // These are the edges where edge.bubble == this bubble.
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
