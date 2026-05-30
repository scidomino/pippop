use crate::game::state::{GameState, InteractContext};
use crate::graph::{bubble::BubbleStyle, Graph};
use crate::graphics::RenderContext;
use macroquad::prelude::{draw_text, set_camera, set_default_camera, KeyCode};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::sync::OnceLock;

#[derive(Default)]
pub struct DebugManager {
    pub open_air_loop_warning: Option<i32>,
}

impl DebugManager {
    pub fn new() -> Self {
        Self::default()
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

    pub fn update(&mut self, state: &GameState) {
        if !is_debug() {
            return;
        }
        self.open_air_loop_warning = calculate_open_air_turning_number(&state.graph);
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
        crate::graphics::bubble::draw_ctrl_points(&ctx.state.graph);

        // Draw FPS in screen space
        set_default_camera();
        draw_text(
            &format!("FPS: {:03}", macroquad::time::get_fps()),
            10.0,
            30.0,
            30.0,
            crate::graphics::colors::WHITE,
        );

        // Draw warning if turning number is not 1
        if let Some(tn) = self.open_air_loop_warning {
            if tn != 1 {
                let warning_text = format!(
                    "WARNING: Open Air Bubble has {} loops (expected 1 CCW loop)!",
                    tn
                );
                draw_text(
                    &warning_text,
                    20.0,
                    macroquad::window::screen_height() - 30.0,
                    30.0,
                    crate::graphics::colors::RED,
                );
            }
        }

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

fn calculate_open_air_turning_number(graph: &Graph) -> Option<i32> {
    // Find open air bubble
    let open_air_key = graph.bubbles.iter().find_map(|(k, b)| {
        if matches!(b.style, BubbleStyle::OpenAir) {
            Some(k)
        } else {
            None
        }
    })?;

    let bubble = &graph.bubbles[open_air_key];
    if bubble.edges.is_empty() {
        return None;
    }

    // Get the list of vertices for the open air bubble
    let mut vertices = Vec::new();
    for &ekey in &bubble.edges {
        let pos = graph.vertices[ekey.vertex].point.position;
        vertices.push(pos);
    }

    let n = vertices.len();
    if n < 3 {
        return Some(1);
    }

    // Calculate edges and their angles
    let mut angles = Vec::with_capacity(n);
    for i in 0..n {
        let p1 = vertices[i];
        let p2 = vertices[(i + 1) % n];
        let edge = p2 - p1;
        angles.push(edge.y.atan2(edge.x));
    }

    // Sum angle differences
    let mut total_turn = 0.0;
    for i in 0..n {
        let a1 = angles[i];
        let a2 = angles[(i + 1) % n];
        let mut diff = a2 - a1;

        // Normalize diff to [-PI, PI]
        while diff > std::f32::consts::PI {
            diff -= 2.0 * std::f32::consts::PI;
        }
        while diff < -std::f32::consts::PI {
            diff += 2.0 * std::f32::consts::PI;
        }

        total_turn += diff;
    }

    let turning_number = (total_turn / (2.0 * std::f32::consts::PI)).round() as i32;
    Some(turning_number)
}

fn is_debug() -> bool {
    static DEBUG_MODE: OnceLock<bool> = OnceLock::new();
    *DEBUG_MODE.get_or_init(|| std::env::var("DEBUG").is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::edge::{EdgeKey, Slot};
    use crate::graph::point::Point;
    use crate::graph::vertex::Vertex;
    use crate::graphics::colors;

    #[test]
    fn test_open_air_turning_number() {
        let graph = Graph::new(
            BubbleStyle::swappable(5),
            BubbleStyle::colored(colors::TURQUOISE),
        );
        let tn = calculate_open_air_turning_number(&graph);
        assert_eq!(tn, Some(1));
    }

    #[test]
    fn test_open_air_turning_number_figure_eight() {
        use macroquad::prelude::vec2;

        let mut graph = Graph::new(
            BubbleStyle::swappable(5),
            BubbleStyle::colored(colors::TURQUOISE),
        );

        let v3 = graph.vertices.insert(Vertex::new(Point::new(0.0, 0.0)));
        let v4 = graph.vertices.insert(Vertex::new(Point::new(0.0, 0.0)));

        let oa_key = graph.get_open_air();

        let keys = vec![
            EdgeKey::new(graph.vertices.keys().nth(0).unwrap(), Slot::A),
            EdgeKey::new(graph.vertices.keys().nth(1).unwrap(), Slot::A),
            EdgeKey::new(v3, Slot::A),
            EdgeKey::new(v4, Slot::A),
        ];

        graph.vertices.inner[keys[0].vertex].point.position = vec2(0.0, 0.0);
        graph.vertices.inner[keys[1].vertex].point.position = vec2(1.0, 1.0);
        graph.vertices.inner[keys[2].vertex].point.position = vec2(1.0, 0.0);
        graph.vertices.inner[keys[3].vertex].point.position = vec2(0.0, 1.0);

        graph.bubbles.inner[oa_key].edges = keys;

        let tn = calculate_open_air_turning_number(&graph);
        assert_eq!(tn, Some(0));
    }
}
