use slotmap::new_key_type;
use super::edge::EdgeKey;

new_key_type! {
    pub struct BubbleKey;
}

#[derive(Debug, Clone)]
pub struct Bubble {
    pub open_air: bool,
    pub edges: Vec<EdgeKey>,
}

impl Bubble {
    pub fn new(open_air: bool) -> Self {
        Bubble {
            open_air,
            edges: Vec::new(),
        }
    }

    pub fn merge(&mut self, other: &Bubble) {
        if other.open_air {
            self.open_air = true;
        }
    }
}
