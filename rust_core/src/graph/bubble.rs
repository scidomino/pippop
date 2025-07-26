// rust_core/src/graph/bubble.rs

use crate::graph::EdgeKey;

#[derive(Debug)]
pub struct Bubble {
    pub first_edge_key: EdgeKey,
    pub pressure: f64,
}
