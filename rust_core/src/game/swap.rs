use crate::graph::bubble::{BubbleKey, BubbleStyle};
use crate::graph::edge::EdgeKey;
use crate::graph::Graph;
use crate::graphics::bubble;
use crate::graphics::geometry;
use macroquad::math::Vec2;

const SWAP_TIME: f32 = 0.2;

pub struct ActiveSwap {
    pub edge: EdgeKey,

    pub swappable_bkey: BubbleKey,

    pub colored_bkey: BubbleKey,
    pub colored_style: BubbleStyle,

    pub progress: f32, // 0.0 to 1.0
    pub start_area: f32,
    pub target_area: f32,
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
            return swap.swappable_bkey == bkey || swap.colored_bkey == bkey;
        }
        false
    }

    pub fn draw(&self, ctx: &crate::graphics::RenderContext) {
        if let Some(swap) = &self.active_swap {
            self.draw_bubbles(ctx, swap);
        }
    }

    fn draw_bubbles(&self, ctx: &crate::graphics::RenderContext, swap: &ActiveSwap) {
        let twin = ctx.graph.vertices.get_edge(swap.edge).twin;

        // 1. Get the shared edge points (wall) from the swappable's perspective
        // Since we rebubbled, the first edge in the bubble's edge list is the shared wall.
        let mut e_points = ctx.graph.vertices.get_edge(swap.edge).points.clone();
        e_points.push(ctx.graph.vertices[twin.vertex].point.position);

        let mut t_points = ctx.graph.vertices.get_edge(twin).points.clone();
        t_points.push(ctx.graph.vertices[swap.edge.vertex].point.position);

        // 2. Get full points for both
        let mut s_points = ctx.graph.bubbles[swap.colored_bkey]
            .edges
            .iter()
            .skip(1)
            .flat_map(|&ekey| ctx.graph.vertices.get_edge(ekey).points.clone())
            .collect::<Vec<Vec2>>();
        s_points.push(ctx.graph.vertices[swap.edge.vertex].point.position);

        let mut p_points = ctx.graph.bubbles[swap.swappable_bkey]
            .edges
            .iter()
            .skip(1)
            .flat_map(|&ekey| ctx.graph.vertices.get_edge(ekey).points.clone())
            .collect::<Vec<Vec2>>();
        p_points.push(ctx.graph.vertices[twin.vertex].point.position);

        if e_points.is_empty() || s_points.is_empty() || p_points.is_empty() {
            return;
        }

        // 3. Create the two parts of the tween
        // Part 1: wall -> swappable
        let part1 = geometry::tween_points(&e_points, &p_points, swap.progress);
        // Part 2: colored -> wall
        let part2 = geometry::tween_points(&s_points, &t_points, swap.progress);

        // 4. Combine into a single smooth polygon
        let mut combined_points = part2;
        combined_points.extend(part1);

        // 5. Calculate a centroid for the label (interpolation of centroids)
        let s_centroid = ctx.graph.bubbles[swap.colored_bkey].centroid;
        let p_centroid = ctx.graph.bubbles[swap.swappable_bkey].centroid;
        let combined_centroid = s_centroid.lerp(p_centroid, swap.progress);

        bubble::draw_bubble(
            &swap.colored_style,
            &combined_points,
            combined_centroid,
            ctx.font,
        );
    }

    pub fn interact(&mut self, graph: &mut Graph, point: Vec2) -> bool {
        // Can't swap if already swapping
        if self.active_swap.is_some() {
            return false;
        }

        // Cannot start a swap if clicking inside the swappable bubble itself
        if let Some(swappable_bkey) = graph.bubbles.get_swappable() {
            if graph.bubbles[swappable_bkey].contains(point, graph) {
                return false;
            }
        }

        if let Some(edge_key) = graph.get_closest_swap_candidate(point) {
            self.start_swap(graph, edge_key);
            return true;
        }

        false
    }

    pub fn update(&mut self, graph: &mut Graph, dt: f32) -> Option<BubbleKey> {
        if let Some(swap) = &mut self.active_swap {
            swap.progress += dt / SWAP_TIME;

            let swappable_bkey = swap.swappable_bkey;
            let colored_bkey = swap.colored_bkey;

            let p = swap.progress.clamp(0.0, 1.0);
            if let BubbleStyle::Swappable { area, .. } = &mut graph.bubbles[colored_bkey].style {
                *area = swap.start_area + (swap.target_area - swap.start_area) * p;
            }
            if let BubbleStyle::Swappable { area, .. } = &mut graph.bubbles[swappable_bkey].style {
                *area = swap.target_area + (swap.start_area - swap.target_area) * p;
            }

            if swap.progress >= 1.0 {
                graph.bubbles[swappable_bkey].style = swap.colored_style;
                // Ensure the final swappable bubble has the exact target area
                if let BubbleStyle::Swappable { area, .. } = &mut graph.bubbles[colored_bkey].style
                {
                    *area = swap.target_area;
                }
                self.active_swap = None;
                return Some(swappable_bkey);
            }
        }

        None
    }

    fn start_swap(&mut self, graph: &mut Graph, edge_key: EdgeKey) {
        let twin_key = graph.vertices.get_edge(edge_key).twin;

        let colored_bkey = graph.vertices.get_edge(edge_key).bubble;
        let swappable_bkey = graph.vertices.get_edge(twin_key).bubble;

        // Align bubble edge lists to start at the shared boundary for smooth tweening
        graph.rebubble(colored_bkey, edge_key);
        graph.rebubble(swappable_bkey, twin_key);

        let colored_style = graph.bubbles[colored_bkey].style;
        let start_area = colored_style.get_target_area();
        let target_area = 3000.0;

        let BubbleStyle::Swappable { swaps_left, .. } = graph.bubbles[swappable_bkey].style else {
            panic!()
        };

        // Immediately apply new swappable style to the colored bubble, starting with colored's area
        graph.bubbles[colored_bkey].style = BubbleStyle::Swappable {
            swaps_left: (swaps_left - 1).max(0),
            area: start_area,
        };

        self.active_swap = Some(ActiveSwap {
            edge: edge_key,
            swappable_bkey,
            colored_bkey,
            colored_style,
            progress: 0.0,
            start_area,
            target_area,
        });
    }
}
