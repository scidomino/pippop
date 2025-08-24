use super::edge::EdgeKey;
use slotmap::new_key_type;

new_key_type! {
    pub struct BubbleKey;
}

#[derive(Debug, Clone)]
pub struct Bubble {
    pub open_air: bool,
    pub edges: Vec<EdgeKey>,
    pub area: f32,
    pub size: f32,
}

impl Bubble {
    pub fn new(open_air: bool) -> Self {
        Bubble {
            open_air,
            edges: Vec::new(),
            area: 0.0,
            size: 1.0,
        }
    }

    pub fn merge(&mut self, other: &Bubble) {
        if other.open_air {
            self.open_air = true;
        }
    }

    pub fn get_pressure(&self, area: f32) -> f32 {
        if self.open_air {
            0.0
        } else {
            3000.0 * self.size.sqrt() / area.max(1.0)
        }
    }
}
