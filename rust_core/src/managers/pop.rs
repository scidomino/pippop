use crate::graph::bubble::{BubbleKey, BubbleStyle};
use crate::graph::Graph;
use crate::managers::burst::BurstManager;
use macroquad::math::{vec2, Vec2};

const POPPING_TIME: f32 = 0.5;
const UNNOTICEABLE_AREA: f32 = 100.0; // Slightly larger than Android to be safe with physics

#[derive(Default)]
pub struct PopManager {
    /// Bubbles currently in the timed "frozen" popping state.
    pub pending_pop: Option<BubbleKey>,
}

impl PopManager {
    pub fn new() -> Self {
        Self { pending_pop: None }
    }

    pub fn is_handling(&self, bkey: BubbleKey) -> bool {
        self.pending_pop == Some(bkey)
    }

    pub fn draw_world(&self, graph: &Graph) {
        if let Some(bkey) = self.pending_pop {
            let bubble = &graph.bubbles[bkey];
            let points = crate::graphics::bubble::get_bubble_points(graph, bkey);
            if points.is_empty() {
                return;
            }

            if let BubbleStyle::Popping { size, timer, .. } = bubble.style {
                let progress = (timer / 0.5).clamp(0.0, 1.0);
                let morphed_points = self.apply_pop_morph(&points, bubble.centroid, size, progress);
                crate::graphics::bubble::draw_bubble_body(&bubble.style, &morphed_points);
            }
        }
    }

    fn apply_pop_morph(
        &self,
        points: &[Vec2],
        centroid: Vec2,
        size: i32,
        progress: f32,
    ) -> Vec<Vec2> {
        let target_area = 3000.0 * (size as f32).sqrt();
        let radius = 5.0 * (target_area / std::f32::consts::PI).sqrt();

        let first_p = points[0];
        let start_angle = (first_p.y - centroid.y).atan2(first_p.x - centroid.x);

        let n = points.len();
        let morph_ratio = progress.powi(2);
        let inv_morph = 1.0 - morph_ratio;

        points
            .iter()
            .enumerate()
            .map(|(i, &p)| {
                let angle = start_angle - (2.0 * std::f32::consts::PI) * (i as f32 / n as f32);
                let circle_p = centroid + vec2(angle.cos() * radius, angle.sin() * radius);
                p * morph_ratio + circle_p * inv_morph
            })
            .collect()
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
    pub fn update(&mut self, graph: &mut Graph, dt: f32) -> bool {
        if let Some(bkey) = self.pending_pop {
            if let Some(bubble) = graph.bubbles.get_mut(bkey) {
                if let BubbleStyle::Popping { timer, .. } = &mut bubble.style {
                    *timer -= dt;
                    if *timer <= 0.0 {
                        // "Pop" happened. Style target area is now 0.
                        self.pending_pop = None;
                        return true;
                    }
                }
            } else {
                self.pending_pop = None;
            }
        }
        false
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
                to_remove.push((ekey, false));
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
                            to_remove.push((graph.vertices.get_edge(ekey).twin, true));
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
