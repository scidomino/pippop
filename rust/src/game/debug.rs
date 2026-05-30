use crate::game::state::{GameState, InteractContext};
use crate::graph::{bubble::BubbleStyle, Graph};
use crate::graphics::RenderContext;
use macroquad::prelude::{draw_text, set_camera, set_default_camera, KeyCode};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::sync::OnceLock;

#[derive(Default)]
pub struct DebugManager;

impl DebugManager {
    pub fn new() -> Self {
        Self
    }

    pub fn interact(&self, ctx: &mut InteractContext) {
        if ctx.interaction.keys_pressed.contains(&KeyCode::D) {
            self.dump(&ctx.state.graph);
        }
    }

    fn dump(&self, graph: &Graph) {
        let dump = graph.dump_state();
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Ok(mut file) = File::create("graph_dump.txt") {
                let _ = file.write_all(dump.as_bytes());
                log::info!("Graph state dumped to graph_dump.txt");
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            log::info!("{}", dump);
        }
    }

    pub fn update(&self, state: &GameState) {
        if !is_debug() {
            return;
        }
        if let Err(e) = self.check(state) {
            let dump = state.graph.dump_state();
            #[cfg(not(target_arch = "wasm32"))]
            {
                if let Ok(mut file) = File::create("debug_fail_dump.txt") {
                    let _ = file.write_all(dump.as_bytes());
                }
            }
            log::error!("Graph State Dumped to debug_fail_dump.txt");
            panic!("Graph Invariant Failure: {e}");
        }
    }

    pub fn draw(&self, ctx: &RenderContext) {
        if !is_debug() {
            return;
        }

        // Draw control points in world space
        set_camera(ctx.camera);
        crate::graphics::bubble::draw_debug_points(&ctx.state.graph);

        // Draw FPS in screen space
        set_default_camera();
        draw_text(
            &format!("FPS: {:03}", macroquad::time::get_fps()),
            10.0,
            30.0,
            30.0,
            crate::graphics::colors::WHITE,
        );

        // Restore camera
        set_camera(ctx.camera);
    }

    pub fn check(&self, state: &GameState) -> Result<(), String> {
        let graph = &state.graph;

        // 1. Map every edge in the graph to the bubble it claims to belong to.
        let mut expected_bubble_edges = HashMap::new();
        for (vkey, vertex) in &graph.vertices {
            for ekey in vkey.edge_keys() {
                let owner = vertex.edge(ekey).bubble;
                if !graph.bubbles.contains_key(owner) {
                    return Err(format!(
                        "Edge {ekey:?} claims to belong to non-existent bubble {owner:?}"
                    ));
                }
                expected_bubble_edges
                    .entry(owner)
                    .or_insert_with(HashSet::new)
                    .insert(ekey);
            }
        }

        let mut open_air_count = 0;

        // 2. Check Bubbles
        for (bkey, bubble) in &graph.bubbles {
            if matches!(bubble.style, BubbleStyle::OpenAir) {
                open_air_count += 1;
            }

            if bubble.edges.is_empty() {
                return Err(format!("Bubble {bkey:?} has no edges"));
            }

            let actual_edges: HashSet<_> = bubble.edges.iter().copied().collect();
            let expected_edges = expected_bubble_edges
                .get(&bkey)
                .cloned()
                .unwrap_or_default();

            if actual_edges != expected_edges {
                return Err(format!(
                    "Edge set mismatch for bubble {bkey:?}. \n  In list: {actual:?}\n  Claiming ownership: {expected:?}",
                    actual = actual_edges,
                    expected = expected_edges
                ));
            }

            for (i, &ekey) in bubble.edges.iter().enumerate() {
                // Check vertex existence (though iteration above already proves existence, we check ekey origin)
                if !graph.vertices.contains_key(ekey.vertex) {
                    return Err(format!(
                        "Edge {ekey:?} in bubble {bkey:?} points to non-existent vertex"
                    ));
                }

                let edge = graph.vertices.get_edge(ekey);

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
            }
        }

        // 3. Check Open Air
        if open_air_count != 1 {
            return Err(format!(
                "Expected exactly 1 OpenAir bubble, found {open_air_count}"
            ));
        }

        // 4. Topology Invariants
        let v_count = graph.vertices.len();
        let b_count = graph.bubbles.len();
        let total_half_edges = v_count * 3;

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

fn is_debug() -> bool {
    static DEBUG_MODE: OnceLock<bool> = OnceLock::new();
    *DEBUG_MODE.get_or_init(|| std::env::var("DEBUG").is_ok())
}
