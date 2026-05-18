use crate::game::state::{GamePhase, UpdateContext};
use crate::graph::bubble::BubbleStyle;
use std::collections::HashMap;

const UNNOTICEABLE_AREA: f32 = 100.0; // Area threshold for removing tiny bubbles

#[derive(Default)]
pub struct ReapManager;

impl ReapManager {
    pub fn new() -> Self {
        Self
    }

    /// Removes popping bubbles that have finished their animation when they are either:
    /// - deflated enough to be removed without causing noticeable visual artifacts, or
    /// - touching the open air.
    pub fn update(&self, ctx: &mut UpdateContext) {
        if matches!(ctx.state.phase, GamePhase::Paused(_)) {
            return;
        }
        let graph = &ctx.state.graph;
        // collect edges to remove. Note that these are always the "twin"
        // edge of the bubble being removed, so that we can merge into
        // the surviving bubble.
        let to_remove: Vec<_> = graph
            .bubbles
            .iter()
            .filter(|(_, b)| matches!(b.style, BubbleStyle::Invisible { size } if size <= 0))
            .filter_map(|(_, bubble)| {
                // Check if it's touching OpenAir
                let touches_open_air = bubble.edges.iter().find_map(|&ekey| {
                    let twin_ekey = graph.vertices.get_edge(ekey).twin;
                    let twin_bubble_key = graph.vertices.get_edge(twin_ekey).bubble;
                    matches!(graph.bubbles[twin_bubble_key].style, BubbleStyle::OpenAir)
                        .then_some(twin_ekey)
                });

                if let Some(ekey) = touches_open_air {
                    return Some(ekey);
                }

                // If not touching open air, check if it's tiny
                if bubble.area < UNNOTICEABLE_AREA {
                    let mut adjacent_count = HashMap::new();
                    for &ekey in &bubble.edges {
                        let twin_bubble_key = graph
                            .vertices
                            .get_edge(graph.vertices.get_edge(ekey).twin)
                            .bubble;
                        *adjacent_count.entry(twin_bubble_key).or_insert(0) += 1;
                    }

                    // Find a neighbor that shares exactly one edge to merge into
                    let neighbor_key = adjacent_count
                        .iter()
                        .find(|&(_, &count)| count == 1)
                        .map(|(&k, _)| k)?;

                    // Find the edge that connects to this neighbor
                    return bubble
                        .edges
                        .iter()
                        .find(|&&e| {
                            graph
                                .vertices
                                .get_edge(graph.vertices.get_edge(e).twin)
                                .bubble
                                == neighbor_key
                        })
                        .map(|&ekey| graph.vertices.get_edge(ekey).twin);
                }
                None
            })
            .collect();

        for ekey in to_remove {
            if ctx.state.graph.vertices.contains_key(ekey.vertex) {
                ctx.state.graph.remove_edge(ekey);
            }
        }
    }
}
