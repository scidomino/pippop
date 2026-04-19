use crate::graph::bubble::{BubbleKey, BubbleStyle};
use crate::graph::Graph;
use crate::managers::burst::BurstManager;

const POPPING_TIME: f32 = 0.5;
const UNOTICEABLE_AREA: f32 = 100.0; // Slightly larger than Android to be safe with physics

#[derive(Default)]
pub struct PopManager {
    /// Bubbles currently in the timed "frozen" popping state.
    pub pending_pop: Option<BubbleKey>,
}

impl PopManager {
    pub fn new() -> Self {
        Self { pending_pop: None }
    }

    /// Checks if any bubble is ready to pop and transitions it to the Popping state.
    pub fn deflate_big_bubble(&mut self, graph: &mut Graph) -> bool {
        if self.pending_pop.is_some() {
            return false;
        }

        if let Some(bkey) =
            graph.bubbles.iter().find_map(
                |(k, b)| {
                    if b.style.is_poppable() {
                        Some(k)
                    } else {
                        None
                    }
                },
            )
        {
            let style = graph.bubbles[bkey].style;
            if let BubbleStyle::Standard { size, color } = style {
                graph.bubbles[bkey].style = BubbleStyle::Popping {
                    size,
                    color,
                    timer: POPPING_TIME,
                };
                self.pending_pop = Some(bkey);
                return true;
            }
        }
        false
    }

    /// Updates timers for popping bubbles and handles transitions.
    pub fn update(&mut self, graph: &mut Graph, dt: f32) {
        if let Some(bkey) = self.pending_pop {
            if let Some(bubble) = graph.bubbles.get_mut(bkey) {
                if let BubbleStyle::Popping { timer, .. } = &mut bubble.style {
                    *timer -= dt;
                    if *timer <= 0.0 {
                        // "Pop" happened. Style target area is now 0.
                        self.pending_pop = None;
                    }
                }
            } else {
                self.pending_pop = None;
            }
        }
    }

    /// Removes bubbles that have effectively deflated.
    pub fn remove_deflated(&mut self, graph: &mut Graph, burst_manager: &BurstManager) {
        let mut to_remove = Vec::new();

        // Identify bubbles that should be removed
        for (_, bubble) in graph.bubbles.iter() {
            if !matches!(bubble.style, BubbleStyle::Popping { timer, .. } if timer <= 0.0) {
                continue;
            }

            // Check if it's small enough or touching OpenAir
            let mut touches_open_air = None;
            let mut adjacent_count = std::collections::HashMap::new();

            for &ekey in &bubble.edges {
                let twin_ekey = graph.get_edge(ekey).twin;
                let twin_bubble_key = graph.get_edge(twin_ekey).bubble;
                let twin_bubble = &graph.bubbles[twin_bubble_key];

                if matches!(twin_bubble.style, BubbleStyle::OpenAir) {
                    touches_open_air = Some(twin_ekey);
                    break;
                }
                *adjacent_count.entry(twin_bubble_key).or_insert(0) += 1;
            }

            if let Some(ekey) = touches_open_air {
                to_remove.push((ekey, false));
            } else {
                // If not touching open air, check if it's tiny
                // We use the area contribution from the physics model for accuracy
                let area = bubble
                    .edges
                    .iter()
                    .map(|&e| graph.get_bezier(e).area())
                    .sum::<f32>();
                if area.abs() < UNOTICEABLE_AREA {
                    // Find a neighbor that shares exactly one edge to merge into
                    if let Some((&neighbor_key, _)) =
                        adjacent_count.iter().find(|&(_, &count)| count == 1)
                    {
                        // Find the edge that connects to this neighbor
                        let edge_to_neighbor = bubble
                            .edges
                            .iter()
                            .find(|&&e| {
                                graph.get_edge(graph.get_edge(e).twin).bubble == neighbor_key
                            })
                            .cloned();

                        if let Some(ekey) = edge_to_neighbor {
                            to_remove.push((graph.get_edge(ekey).twin, true));
                        }
                    }
                }
            }
        }

        for (ekey, trigger_burst) in to_remove {
            if graph.vertices.contains_key(ekey.vertex) {
                graph.remove_edge(ekey);
                if trigger_burst {
                    burst_manager.burst_all(graph);
                }
            }
        }
    }
}
