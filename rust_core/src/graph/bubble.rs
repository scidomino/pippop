// rust_core/src/graph/bubble.rs

use crate::graph::EdgeKey;
use crate::geom::color::Color;

#[derive(Debug, Clone, Copy)]
pub enum BubbleKind {
    Game { color: Color, size: u32 },
    Player,
    Empty,
}

#[derive(Debug)]
pub struct Bubble {
    pub first_edge_key: EdgeKey,
    pub pressure: f64,
    pub kind: BubbleKind,
}