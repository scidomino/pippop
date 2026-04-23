use crate::graph::bubble::{BubbleKey, BubbleStyle};
use crate::graph::edge::EdgeKey;
use crate::graph::Graph;
use crate::graphics::bubble;
use crate::graphics::geometry;
use macroquad::math::Vec2;

const SWAP_TIME: f32 = 0.2; // 200ms in Android

pub struct ActiveSwap {
    pub edge: EdgeKey,
    pub top_bkey: BubbleKey,
    pub bottom_bkey: BubbleKey,
    pub top_style: BubbleStyle,
    pub bottom_style: BubbleStyle,
    pub return_trip: bool,
    pub progress: f32, // 0.0 to 1.0
}

#[derive(Default)]
pub struct SwapManager {
    pub active_swap: Option<ActiveSwap>,
}

impl SwapManager {
    pub fn new() -> Self {
        Self { active_swap: None }
    }

    pub fn is_handling(&self, bkey: BubbleKey) -> bool {
        if let Some(swap) = &self.active_swap {
            return swap.top_bkey == bkey || swap.bottom_bkey == bkey;
        }
        false
    }

    pub fn draw_world(&self, graph: &Graph, font: &macroquad::text::Font) {
        if let Some(swap) = &self.active_swap {
            let center = graph.vertices.get_edge(swap.edge).point.position;
            self.draw_swapping_bubbles(graph, swap, center, font);
        }
    }

    fn draw_swapping_bubbles(
        &self,
        graph: &Graph,
        swap: &ActiveSwap,
        center: Vec2,
        font: &macroquad::text::Font,
    ) {
        let top_points = bubble::get_bubble_points(graph, swap.top_bkey);
        let bottom_points = bubble::get_bubble_points(graph, swap.bottom_bkey);

        if top_points.is_empty() || bottom_points.is_empty() {
            return;
        }

        // Top Bubble Animation
        let rotated_top_start =
            geometry::rotate_points(&top_points, center, swap.progress * std::f32::consts::PI);
        let rotated_top_end = geometry::rotate_points(
            &bottom_points,
            center,
            (swap.progress - 1.0) * std::f32::consts::PI,
        );
        let morphed_top =
            geometry::tween_points(&rotated_top_start, &rotated_top_end, swap.progress);

        // Bottom Bubble Animation
        let rotated_bottom_start =
            geometry::rotate_points(&bottom_points, center, swap.progress * std::f32::consts::PI);
        let rotated_bottom_end = geometry::rotate_points(
            &top_points,
            center,
            (swap.progress - 1.0) * std::f32::consts::PI,
        );
        let morphed_bottom =
            geometry::tween_points(&rotated_bottom_start, &rotated_bottom_end, swap.progress);

        bubble::draw_bubble_body(&swap.top_style, &morphed_top);
        bubble::draw_bubble_body(&swap.bottom_style, &morphed_bottom);

        // Draw Labels at rotated centroids
        let top_centroid = graph.bubbles[swap.top_bkey].centroid;
        let bottom_centroid = graph.bubbles[swap.bottom_bkey].centroid;

        let rotated_top_centroid = geometry::rotate_points(
            &[top_centroid],
            center,
            swap.progress * std::f32::consts::PI,
        )[0];
        let rotated_bottom_centroid = geometry::rotate_points(
            &[bottom_centroid],
            center,
            swap.progress * std::f32::consts::PI,
        )[0];

        crate::graphics::ui::draw_bubble_label(&swap.top_style, rotated_top_centroid, font);
        crate::graphics::ui::draw_bubble_label(&swap.bottom_style, rotated_bottom_centroid, font);
    }

    pub fn otter_swap(&mut self, graph: &mut Graph, point: Vec2) -> bool {
        // Can't swap if already swapping
        if self.active_swap.is_some() {
            return false;
        }

        // Cannot start a swap if clicking inside the player bubble itself
        if let Some(player_bkey) = graph.bubbles.get_player_bubble() {
            if graph.bubbles[player_bkey].contains(point, graph) {
                return false;
            }
        }

        if let Some(edge_key) = graph.get_closest_otter_swappable(point) {
            self.start_swap(graph, edge_key, false);
            return true;
        }

        false
    }

    pub fn update(&mut self, graph: &mut Graph, dt: f32) -> bool {
        if let Some(swap) = &mut self.active_swap {
            swap.progress += dt / SWAP_TIME;

            let top_bkey = swap.top_bkey;
            let bottom_bkey = swap.bottom_bkey;

            if swap.progress >= 1.0 {
                // Perform the final style switch
                graph.bubbles[top_bkey].style = swap.bottom_style;
                graph.bubbles[bottom_bkey].style = swap.top_style;

                self.active_swap = None;
                return true;
            } else {
                // Update the progress in the waiting styles for physics interpolation
                let top_target_area = swap.top_style.get_target_area();
                let bottom_target_area = swap.bottom_style.get_target_area();

                graph.bubbles[top_bkey].style = BubbleStyle::Waiting {
                    start_area: top_target_area,
                    end_area: bottom_target_area,
                    progress: swap.progress,
                };
                graph.bubbles[bottom_bkey].style = BubbleStyle::Waiting {
                    start_area: bottom_target_area,
                    end_area: top_target_area,
                    progress: swap.progress,
                };
            }
        }
        false
    }

    fn start_swap(&mut self, graph: &mut Graph, edge_key: EdgeKey, return_trip: bool) {
        let twin_key = graph.vertices.get_edge(edge_key).twin;

        let bottom_bkey = graph.vertices.get_edge(edge_key).bubble;
        let top_bkey = graph.vertices.get_edge(twin_key).bubble;

        // Align bubble edge lists to start at the shared boundary for smooth tweening
        graph.rebubble(bottom_bkey, edge_key);
        graph.rebubble(top_bkey, twin_key);

        let bottom_style = graph.bubbles[bottom_bkey].style;
        let top_style = graph.bubbles[top_bkey].style;

        let top_target_area = top_style.get_target_area();
        let bottom_target_area = bottom_style.get_target_area();

        graph.bubbles[top_bkey].style = BubbleStyle::Waiting {
            start_area: top_target_area,
            end_area: bottom_target_area,
            progress: 0.0,
        };

        graph.bubbles[bottom_bkey].style = BubbleStyle::Waiting {
            start_area: bottom_target_area,
            end_area: top_target_area,
            progress: 0.0,
        };

        self.active_swap = Some(ActiveSwap {
            edge: edge_key,
            top_bkey,
            bottom_bkey,
            top_style,
            bottom_style,
            return_trip,
            progress: 0.0,
        });
    }
}
