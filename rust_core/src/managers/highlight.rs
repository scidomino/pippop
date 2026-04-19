use crate::graph::bubble::BubbleStyle;
use crate::graph::Graph;
use macroquad::math::Vec2;

const TEASER_DELAY: f32 = 4.0;
const TEASER_THROB: f32 = 1.0;

pub struct HighlightManager {
    pub point: Option<Vec2>,
    pub time: f32,
}

impl Default for HighlightManager {
    fn default() -> Self {
        Self::new()
    }
}

impl HighlightManager {
    pub fn new() -> Self {
        Self {
            point: None,
            time: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.time += dt;
    }

    pub fn set_point(&mut self, point: Option<Vec2>) {
        self.point = point;
        self.time = 0.0;
    }

    pub fn get_glow_requests(&self, graph: &Graph) -> Vec<(crate::graph::bubble::BubbleKey, f32)> {
        let mut requests = Vec::new();

        if let Some(p) = self.point {
            if let Some(ekey) = graph.get_closest_otter_swappable(p) {
                let bkey = graph.get_edge(ekey).bubble;
                requests.push((bkey, 1.0));
            }
        } else {
            let cycle_time = self.time % TEASER_DELAY;
            if cycle_time > TEASER_DELAY / 2.0 {
                let ratio = (self.time * std::f32::consts::PI * 2.0 / TEASER_THROB)
                    .sin()
                    .powi(2);

                if let Some(player_bkey) = graph.get_player_bubble() {
                    let player_bubble = &graph.bubbles[player_bkey];
                    for &ekey in &player_bubble.edges {
                        let twin_bkey = graph.get_edge(graph.get_edge(ekey).twin).bubble;
                        if !matches!(graph.bubbles[twin_bkey].style, BubbleStyle::OpenAir) {
                            requests.push((twin_bkey, ratio));
                        }
                    }
                }
            }
        }

        requests
    }
}
