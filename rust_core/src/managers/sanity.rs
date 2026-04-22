use crate::graph::{bubble::BubbleStyle, edge::EdgeKey, Graph};
use std::collections::HashSet;

pub struct SanityManager;

impl Default for SanityManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SanityManager {
    pub fn new() -> Self {
        Self
    }

    pub fn check_invariants(&self, graph: &Graph) -> Result<(), String> {
        let mut seen_edges = HashSet::new();
        let mut open_air_count = 0;

        // 1. Check Bubbles
        for (bkey, bubble) in &graph.bubbles {
            if matches!(bubble.style, BubbleStyle::OpenAir) {
                open_air_count += 1;
            }

            if bubble.edges.is_empty() {
                return Err(format!("Bubble {:?} has no edges", bkey));
            }

            for (i, &ekey) in bubble.edges.iter().enumerate() {
                // Check vertex existence
                if !graph.vertices.contains_key(ekey.vertex) {
                    return Err(format!(
                        "Edge {:?} in bubble {:?} points to non-existent vertex",
                        ekey, bkey
                    ));
                }

                let edge = graph.vertices.get_edge(ekey);

                // Check bubble ownership
                if edge.bubble != bkey {
                    return Err(format!(
                        "Edge {:?} in bubble {:?} thinks it belongs to bubble {:?}",
                        ekey, bkey, edge.bubble
                    ));
                }

                // Check twin consistency
                let tkey = edge.twin;
                if !graph.vertices.contains_key(tkey.vertex) {
                    return Err(format!(
                        "Edge {:?} in bubble {:?} has twin {:?} pointing to non-existent vertex",
                        ekey, bkey, tkey
                    ));
                }
                if graph.vertices.get_edge(tkey).twin != ekey {
                    return Err(format!(
                        "Twin inconsistency: {:?}.twin = {:?}, but {:?}.twin = {:?}",
                        ekey,
                        tkey,
                        tkey,
                        graph.vertices.get_edge(tkey).twin
                    ));
                }

                // Check continuity (next_on_bubble should be the next edge in the list)
                // Note: next_on_bubble is private to Graph but we can call it if we are in the same crate
                // Actually, let's see if it's pub. It was NOT pub in the previous read.
                // I should check if I need to make it pub or move the logic here.
                let next_ekey = graph.vertices.next_on_bubble(ekey);
                let expected_next = bubble.edges[(i + 1) % bubble.edges.len()];
                if next_ekey != expected_next {
                    return Err(format!(
                        "Continuity error in bubble {:?}: edge {:?} is followed by {:?} in list, but next_on_bubble is {:?}",
                        bkey, ekey, expected_next, next_ekey
                    ));
                }

                if !seen_edges.insert(ekey) {
                    return Err(format!(
                        "Edge {:?} appears in more than one bubble (or twice in one)",
                        ekey
                    ));
                }
            }
        }

        // 2. Check Open Air
        if open_air_count != 1 {
            return Err(format!(
                "Expected exactly 1 OpenAir bubble, found {}",
                open_air_count
            ));
        }

        // 3. Check Edge Completeness
        let total_half_edges = graph.vertices.len() * 3;
        if seen_edges.len() != total_half_edges {
            // Find which edges are missing
            for (vkey, _) in &graph.vertices {
                for i in 0..3 {
                    let ekey = EdgeKey::new(vkey, i as u8);
                    if !seen_edges.contains(&ekey) {
                        return Err(format!("Edge {:?} is not owned by any bubble", ekey));
                    }
                }
            }
        }

        // 4. Topology Invariants (Honeycomb)
        let v_count = graph.vertices.len();
        let b_count = graph.bubbles.len();

        if v_count != (b_count - 2) * 2 {
            return Err(format!(
                "Topology invariant failed: vertices({}) != (bubbles({}) - 2) * 2",
                v_count, b_count
            ));
        }

        if total_half_edges != 6 * (b_count - 2) {
            return Err(format!(
                "Topology invariant failed: half_edges({}) != 6 * (bubbles({}) - 2)",
                total_half_edges, b_count
            ));
        }

        Ok(())
    }
}
