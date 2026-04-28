use crate::graph::bubble::BubbleStyle;
use crate::graph::Graph;

const UNNOTICEABLE_AREA: f32 = 100.0; // Slightly larger than Android to be safe with physics

#[derive(Default)]
pub struct ReapManager;

impl ReapManager {
    pub fn new() -> Self {
        Self
    }

    /// Removes popping bubbles that have finished their animation when they are either:
    /// - deflated enough to be removed without causing noticeable visual artifacts, or
    /// - touching the open air.
    pub fn reap_popped(&self, graph: &mut Graph) {
        let mut to_remove = Vec::new();

        // Identify bubbles that should be removed
        for (_, bubble) in graph.bubbles.iter() {
            if !matches!(bubble.style, BubbleStyle::Popping { timer, .. } if timer <= 0.0) {
                continue;
            }

            // Check if it's touching OpenAir
            let mut touches_open_air = None;
            let mut adjacent_count = std::collections::HashMap::new();

            for &ekey in &bubble.edges {
                let twin_ekey = graph.vertices.get_edge(ekey).twin;
                let twin_bubble_key = graph.vertices.get_edge(twin_ekey).bubble;
                let twin_bubble = &graph.bubbles[twin_bubble_key];

                if matches!(twin_bubble.style, BubbleStyle::OpenAir) {
                    touches_open_air = Some(twin_ekey);
                    break;
                }
                *adjacent_count.entry(twin_bubble_key).or_insert(0) += 1;
            }

            if let Some(ekey) = touches_open_air {
                to_remove.push(ekey);
            } else {
                // If not touching open air, check if it's tiny
                let area = bubble.area;
                if area < UNNOTICEABLE_AREA {
                    // Find a neighbor that shares exactly one edge to merge into
                    if let Some((&neighbor_key, _)) =
                        adjacent_count.iter().find(|&(_, &count)| count == 1)
                    {
                        // Find the edge that connects to this neighbor
                        let edge_to_neighbor = bubble
                            .edges
                            .iter()
                            .find(|&&e| {
                                graph
                                    .vertices
                                    .get_edge(graph.vertices.get_edge(e).twin)
                                    .bubble
                                    == neighbor_key
                            })
                            .cloned();

                        if let Some(ekey) = edge_to_neighbor {
                            to_remove.push(graph.vertices.get_edge(ekey).twin);
                        }
                    }
                }
            }
        }

        for ekey in to_remove {
            if graph.vertices.contains_key(ekey.vertex) {
                graph.remove_edge(ekey);
            }
        }
    }
}
