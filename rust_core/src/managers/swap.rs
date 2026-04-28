use crate::graph::bubble::{BubbleKey, BubbleStyle};
use crate::graph::edge::EdgeKey;
use crate::graph::Graph;
use crate::graphics::bubble;
use crate::graphics::geometry;
use macroquad::math::Vec2;

const SWAP_TIME: f32 = 0.2;

pub struct ActiveSwap {
    pub edge: EdgeKey,
    pub twin: EdgeKey,
    pub player_bkey: BubbleKey,
    pub nonplayer_bkey: BubbleKey,
    pub nonplayer_style: BubbleStyle,
    pub player_style: BubbleStyle,
    pub new_player_style: BubbleStyle,
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
            return swap.player_bkey == bkey || swap.nonplayer_bkey == bkey;
        }
        false
    }

    pub fn draw(&self, ctx: &crate::graphics::RenderContext) {
        if let Some(swap) = &self.active_swap {
            self.draw_bubbles(ctx, swap);
        }
    }

    fn draw_bubbles(&self, ctx: &crate::graphics::RenderContext, swap: &ActiveSwap) {
        // 1. Get the shared edge points (wall) from the non-player's perspective
        // Since we rebubbled, the first edge in the bubble's edge list is the shared wall.
        let mut e_points = ctx.graph.vertices.get_edge(swap.edge).points.clone();
        e_points.push(ctx.graph.vertices[swap.twin.vertex].point.position);

        let mut t_points = ctx.graph.vertices.get_edge(swap.twin).points.clone();
        t_points.push(ctx.graph.vertices[swap.edge.vertex].point.position);

        // 2. Get full points for both
        let mut np_points = ctx.graph.bubbles[swap.nonplayer_bkey]
            .edges
            .iter()
            .skip(1)
            .flat_map(|&ekey| ctx.graph.vertices.get_edge(ekey).points.clone())
            .collect::<Vec<Vec2>>();
        np_points.push(ctx.graph.vertices[swap.edge.vertex].point.position);

        let mut p_points = ctx.graph.bubbles[swap.player_bkey]
            .edges
            .iter()
            .skip(1)
            .flat_map(|&ekey| ctx.graph.vertices.get_edge(ekey).points.clone())
            .collect::<Vec<Vec2>>();
        p_points.push(ctx.graph.vertices[swap.twin.vertex].point.position);

        if e_points.is_empty() || np_points.is_empty() || p_points.is_empty() {
            return;
        }

        // 3. Create the two parts of the tween
        // Part 1: wall -> player
        let part1 = geometry::tween_points(&e_points, &p_points, swap.progress);
        // Part 2: nonplayer -> wall
        let part2 = geometry::tween_points(&np_points, &t_points, swap.progress);

        // 4. Combine into a single smooth polygon
        let mut combined_points = part2;
        combined_points.extend(part1);

        // 5. Calculate a centroid for the label (interpolation of centroids)
        let np_centroid = ctx.graph.bubbles[swap.nonplayer_bkey].centroid;
        let p_centroid = ctx.graph.bubbles[swap.player_bkey].centroid;
        let combined_centroid = np_centroid.lerp(p_centroid, swap.progress);

        bubble::draw_bubble(
            &swap.player_style,
            &crate::graphics::bubble::get_points_for_bubble(
                ctx.graph,
                &ctx.graph.bubbles[swap.player_bkey],
            ),
            ctx.graph.bubbles[swap.player_bkey].centroid,
            ctx.font,
        );

        bubble::draw_bubble(
            &swap.new_player_style,
            &crate::graphics::bubble::get_points_for_bubble(
                ctx.graph,
                &ctx.graph.bubbles[swap.nonplayer_bkey],
            ),
            ctx.graph.bubbles[swap.nonplayer_bkey].centroid,
            ctx.font,
        );

        bubble::draw_bubble(
            &swap.nonplayer_style,
            &combined_points,
            combined_centroid,
            ctx.font,
        );
    }

    pub fn swap(&mut self, graph: &mut Graph, point: Vec2) -> bool {
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
            self.start_swap(graph, edge_key);
            return true;
        }

        false
    }

    pub fn update(&mut self, graph: &mut Graph, dt: f32) -> bool {
        if let Some(swap) = &mut self.active_swap {
            swap.progress += dt / SWAP_TIME;

            let player_bkey = swap.player_bkey;
            let nonplayer_bkey = swap.nonplayer_bkey;

            if swap.progress >= 1.0 {
                graph.bubbles[player_bkey].style = swap.nonplayer_style;
                graph.bubbles[nonplayer_bkey].style = swap.new_player_style;

                self.active_swap = None;
                return true;
            } else {
                // Update the progress in the waiting styles for physics interpolation
                let np_target_area = swap.nonplayer_style.get_target_area();
                let p_target_area = swap.player_style.get_target_area();

                graph.bubbles[nonplayer_bkey].style = BubbleStyle::Waiting {
                    start_area: np_target_area,
                    end_area: p_target_area,
                    progress: swap.progress,
                };
                graph.bubbles[player_bkey].style = BubbleStyle::Waiting {
                    start_area: p_target_area,
                    end_area: np_target_area,
                    progress: swap.progress,
                };
            }
        }
        false
    }

    fn start_swap(&mut self, graph: &mut Graph, edge_key: EdgeKey) {
        let twin_key = graph.vertices.get_edge(edge_key).twin;

        let nonplayer_bkey = graph.vertices.get_edge(edge_key).bubble;
        let player_bkey = graph.vertices.get_edge(twin_key).bubble;

        // Align bubble edge lists to start at the shared boundary for smooth tweening
        graph.rebubble(nonplayer_bkey, edge_key);
        graph.rebubble(player_bkey, twin_key);

        let nonplayer_style = graph.bubbles[nonplayer_bkey].style;
        let player_style = graph.bubbles[player_bkey].style;
        let new_player_style = match player_style {
            BubbleStyle::Player { swaps_left } => BubbleStyle::Player {
                swaps_left: (swaps_left - 1).max(0),
            },
            other => other,
        };

        let p_target_area = player_style.get_target_area();
        let np_target_area = nonplayer_style.get_target_area();

        graph.bubbles[nonplayer_bkey].style = BubbleStyle::Waiting {
            start_area: np_target_area,
            end_area: p_target_area,
            progress: 0.0,
        };

        graph.bubbles[player_bkey].style = BubbleStyle::Waiting {
            start_area: p_target_area,
            end_area: np_target_area,
            progress: 0.0,
        };

        self.active_swap = Some(ActiveSwap {
            edge: edge_key,
            twin: twin_key,
            player_bkey,
            nonplayer_bkey,
            nonplayer_style,
            player_style,
            new_player_style,
            progress: 0.0,
        });
    }
}
