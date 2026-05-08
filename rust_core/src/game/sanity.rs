use crate::game::state::GameState;
use crate::graph::{
    bubble::BubbleStyle,
    edge::{EdgeKey, Slot},
};
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;

#[derive(Default)]
pub struct SanityManager;

impl SanityManager {
    pub fn new() -> Self {
        Self
    }

    pub fn update(&self, state: &GameState) {
        if let Err(e) = self.check(state) {
            let dump = state.graph.dump_state();
            #[cfg(not(target_arch = "wasm32"))]
            {
                if let Ok(mut file) = File::create("sanity_fail_dump.txt") {
                    let _ = file.write_all(dump.as_bytes());
                }
            }
            log::error!("Graph State Dumped to sanity_fail_dump.txt");
            panic!("Graph Invariant Failure: {e}");
        }
    }

    pub fn check(&self, state: &GameState) -> Result<(), String> {
        let graph = &state.graph;
        let mut seen_edges = HashSet::new();
        let mut open_air_count = 0;

        // 1. Check Bubbles
        for (bkey, bubble) in &graph.bubbles {
            if matches!(bubble.style, BubbleStyle::OpenAir) {
                open_air_count += 1;
            }

            if bubble.edges.is_empty() {
                return Err(format!("Bubble {bkey:?} has no edges"));
            }

            for (i, &ekey) in bubble.edges.iter().enumerate() {
                // Check vertex existence
                if !graph.vertices.contains_key(ekey.vertex) {
                    return Err(format!(
                        "Edge {ekey:?} in bubble {bkey:?} points to non-existent vertex"
                    ));
                }

                let edge = graph.vertices.get_edge(ekey);

                // Check bubble ownership
                if edge.bubble != bkey {
                    return Err(format!(
                        "Edge {ekey:?} in bubble {bkey:?} thinks it belongs to bubble {bubble_owner:?}",
                        bubble_owner = edge.bubble
                    ));
                }

                // Check twin consistency
                let tkey = edge.twin;
                if !graph.vertices.contains_key(tkey.vertex) {
                    return Err(format!(
                        "Edge {ekey:?} in bubble {bkey:?} has twin {tkey:?} pointing to non-existent vertex"
                    ));
                }
                if graph.vertices.get_edge(tkey).twin != ekey {
                    return Err(format!(
                        "Twin inconsistency: {ekey:?}.twin = {tkey:?}, but {tkey:?}.twin = {twin_of_twin:?}",
                        twin_of_twin = graph.vertices.get_edge(tkey).twin
                    ));
                }

                // Check continuity (next_on_bubble should be the next edge in the list)
                let next_ekey = graph.vertices.next_on_bubble(ekey);
                let expected_next = bubble.edges[(i + 1) % bubble.edges.len()];
                if next_ekey != expected_next {
                    return Err(format!(
                        "Continuity error in bubble {bkey:?}: edge {ekey:?} is followed by {expected_next:?} in list, but next_on_bubble is {next_ekey:?}"
                    ));
                }

                if !seen_edges.insert(ekey) {
                    return Err(format!(
                        "Edge {ekey:?} appears in more than one bubble (or twice in one)"
                    ));
                }
            }
        }

        // 2. Check Open Air
        if open_air_count != 1 {
            return Err(format!(
                "Expected exactly 1 OpenAir bubble, found {open_air_count}"
            ));
        }

        // 3. Check Edge Completeness
        let total_half_edges = graph.vertices.len() * 3;
        if seen_edges.len() != total_half_edges {
            // Find which edges are missing
            for (vkey, _) in &graph.vertices {
                for slot in Slot::all() {
                    let ekey = EdgeKey::new(vkey, slot);
                    if !seen_edges.contains(&ekey) {
                        return Err(format!("Edge {ekey:?} is not owned by any bubble"));
                    }
                }
            }
        }

        // 4. Topology Invariants
        let v_count = graph.vertices.len();
        let b_count = graph.bubbles.len();

        if v_count != (b_count - 2) * 2 {
            return Err(format!(
                "Topology invariant failed: vertices({v_count}) != (bubbles({b_count}) - 2) * 2"
            ));
        }

        if total_half_edges != 6 * (b_count - 2) {
            return Err(format!(
                "Topology invariant failed: half_edges({total_half_edges}) != 6 * (bubbles({b_count}) - 2)"
            ));
        }

        Ok(())
    }
}
