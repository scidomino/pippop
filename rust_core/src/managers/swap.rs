use crate::graph::bubble::{BubbleKey, BubbleStyle};
use crate::graph::edge::EdgeKey;
use crate::graph::Graph;
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
