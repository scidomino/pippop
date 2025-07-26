// rust_core/src/graph/mod.rs

use slotmap::SlotMap;

// Re-export the key types for easier use throughout the crate.
pub use self::keys::{VertexKey, EdgeKey, BubbleKey};
pub use self::vertex::Vertex;
pub use self::edge::Edge;
pub use self::bubble::Bubble;

// Define key types for type safety.
mod keys {
    use slotmap::new_key_type;
    new_key_type! {
        pub struct VertexKey;
        pub struct EdgeKey;
        pub struct BubbleKey;
    }
}

pub mod vertex;
pub mod edge;
pub mod bubble;

#[derive(Debug, Default)]
pub struct Graph {
    pub vertices: SlotMap<VertexKey, Vertex>,
    pub edges: SlotMap<EdgeKey, Edge>,
    pub bubbles: SlotMap<BubbleKey, Bubble>,
}

impl Graph {
    pub fn new() -> Self {
        Graph::default()
    }
}