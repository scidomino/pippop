use crate::graph::bubble::{BubbleKey, BubbleStyle};
use crate::graph::edge::EdgeKey;
use crate::graph::Graph;
use crate::graphics::bubble;
use crate::graphics::colors;
use crate::graphics::geometry;
use macroquad::prelude::*;
use std::collections::HashSet;

pub struct BurstManager {
    threshold: usize,
    /// The edge currently being "frozen" for the burst animation.
    pub active_edge: Option<EdgeKey>,
    pub timer: f32,
    freeze_duration: f32,
}

impl BurstManager {
    pub fn new(threshold: usize) -> Self {
        Self {
            threshold,
            active_edge: None,
            timer: 0.0,
            freeze_duration: 0.5, // 500ms
        }
    }

    pub fn draw(&self, graph: &Graph) {
        if let Some(ekey) = self.active_edge {
            let mut points = Vec::with_capacity(12);
            bubble::push_edge_points(graph, ekey, &mut points);
            let twin_key = graph.vertices.get_edge(ekey).twin;
            points.push(graph.vertices[twin_key.vertex].point.position);

            let progress = 1.0 - (self.timer / 0.5).clamp(0.0, 1.0);
            let width = 40.0 * progress;
            let glow_mesh = geometry::generate_glow_mesh(&points, width, colors::WHITE, false);
            draw_mesh(&glow_mesh);
        }
    }

    pub fn update(&mut self, dt: f32) -> bool {
        if self.active_edge.is_some() {
            self.timer -= dt;
            if self.timer <= 0.0 {
                return true; // Signal that the active edge is ready to be burst
            }
        }
        false
    }

    /// Recursively bursts all matching bubbles in the graph immediately.
    pub fn burst_all(&self, graph: &mut Graph) {
        while let Some(ekey) = self.find_burst_starter(graph) {
            let bkey = graph.vertices.get_edge(ekey).bubble;
            self.burst(graph, ekey);

            // After one burst, the bubble that "won" might have new burstable neighbors.
            // We find a neighbor of the merged bubble and keep going.
            while let Some(next_ekey) = self.find_burstable_edge_in_bubble(graph, bkey) {
                self.burst(graph, next_ekey);
            }
        }
    }

    /// Performs the topological merge of two bubbles across a shared edge.
    pub fn burst(&self, graph: &mut Graph, ekey: EdgeKey) {
        if !self.is_burstable(graph, ekey) {
            return;
        }
        graph.remove_edge(ekey);
    }

    /// Finds a bubble that has at least `threshold` burstable edges.
    pub fn find_burst_starter(&self, graph: &Graph) -> Option<EdgeKey> {
        if graph.bubbles.len() <= 3 {
            return None;
        }

        graph.bubbles.keys().find_map(|bkey| {
            let burstable = self.find_all_burstable_in_bubble(graph, bkey);
            (burstable.len() >= self.threshold)
                .then(|| burstable.into_iter().next())
                .flatten()
        })
    }

    /// Sets up the manager to process a burst with a freeze delay.
    pub fn find_and_set_burstable_edge(&mut self, graph: &Graph) -> bool {
        if let Some(ekey) = self.find_burst_starter(graph) {
            self.active_edge = Some(ekey);
            self.timer = self.freeze_duration;
            return true;
        }
        false
    }

    /// Continues the burst sequence for a specific bubble.
    pub fn find_and_set_next_burstable(&mut self, graph: &Graph, bkey: BubbleKey) -> bool {
        if let Some(ekey) = self.find_burstable_edge_in_bubble(graph, bkey) {
            self.active_edge = Some(ekey);
            self.timer = self.freeze_duration;
            return true;
        }
        false
    }

    fn find_burstable_edge_in_bubble(&self, graph: &Graph, bkey: BubbleKey) -> Option<EdgeKey> {
        let bubble = graph.bubbles.get(bkey)?;
        for &ekey in &bubble.edges {
            if self.is_burstable(graph, ekey) {
                return Some(ekey);
            }
        }
        None
    }

    fn find_all_burstable_in_bubble(&self, graph: &Graph, bkey: BubbleKey) -> HashSet<EdgeKey> {
        graph
            .bubbles
            .get(bkey)
            .map(|bubble| {
                bubble
                    .edges
                    .iter()
                    .filter(|&&ekey| self.is_burstable(graph, ekey))
                    .copied()
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn is_burstable(&self, graph: &Graph, ekey: EdgeKey) -> bool {
        let edge = graph.vertices.get_edge(ekey);
        let twin = graph.vertices.get_edge(edge.twin);

        if edge.bubble == twin.bubble {
            return false;
        }

        let s1 = &graph.bubbles[edge.bubble].style;
        let s2 = &graph.bubbles[twin.bubble].style;

        match (s1, s2) {
            (BubbleStyle::Standard { color: c1, .. }, BubbleStyle::Standard { color: c2, .. }) => {
                c1 == c2
            }
            _ => false,
        }
    }
}
