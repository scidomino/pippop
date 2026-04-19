use super::edge::EdgeKey;
use macroquad::math::Vec2;
use macroquad::prelude::Color;
use slotmap::new_key_type;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BubbleStyle {
    Standard {
        size: i32,
        max_size: i32,
        color: Color,
    },
    Player,
    OpenAir,
}

impl BubbleStyle {
    pub fn merge(&self, other: &BubbleStyle) -> BubbleStyle {
        if *self == BubbleStyle::OpenAir || *other == BubbleStyle::OpenAir {
            return BubbleStyle::OpenAir;
        }

        match (self, other) {
            (
                BubbleStyle::Standard {
                    size: s1,
                    max_size: m1,
                    color,
                },
                BubbleStyle::Standard { size: s2, .. },
            ) => BubbleStyle::Standard {
                size: s1 + s2,
                max_size: *m1,
                color: *color,
            },
            _ => unreachable!("merge should only be called with Standard or OpenAir styles"),
        }
    }
}

new_key_type! {
    pub struct BubbleKey;
}

/// Represents a face in the planar graph (a single bubble or the open air).
///
/// A bubble is defined topologically by a boundary of half-edges.
#[derive(Debug, Clone)]
pub struct Bubble {
    pub style: BubbleStyle,
    /// A clockwise sequence of half-edges that form the continuous boundary 
    /// of this bubble. For every edge in this list, `edge.bubble == this bubble`.
    pub edges: Vec<EdgeKey>,
}

impl Bubble {
    pub fn new(style: BubbleStyle) -> Self {
        Bubble {
            style,
            edges: Vec::new(),
        }
    }

    pub fn merge(&mut self, other: &Bubble) {
        self.style = self.style.merge(&other.style);
    }

    pub fn contains(&self, point: Vec2, graph: &crate::graph::Graph) -> bool {
        let points = crate::graphics::bubble::get_points_for_bubble(graph, self);
        let mut inside = false;
        let mut j = points.len() - 1;
        for i in 0..points.len() {
            if ((points[i].y > point.y) != (points[j].y > point.y))
                && (point.x
                    < (points[j].x - points[i].x) * (point.y - points[i].y)
                        / (points[j].y - points[i].y)
                        + points[i].x)
            {
                inside = !inside;
            }
            j = i;
        }
        inside
    }

    pub fn get_pressure(&self, area: f32) -> f32 {
        match self.style {
            BubbleStyle::Standard { size, .. } => 3000.0 * (size as f32).sqrt() / area.max(100.0),
            BubbleStyle::Player => 3000.0 / area.max(100.0),
            BubbleStyle::OpenAir => 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Graph;
    use macroquad::math::Vec2;

    #[test]
    fn test_bubble_contains() {
        let mut graph = Graph::new();
        graph.init();
        
        // Run physics a few frames to expand the bubbles from their initial degenerate state
        for _ in 0..10 {
            crate::physics::advance_frame(&mut graph);
        }
        
        let bkey = graph.bubbles.keys().next().unwrap();
        let bubble = &graph.bubbles[bkey];
        
        // The centroid of a valid bubble should be inside it
        let centroid = crate::graphics::geometry::calculate_centroid(&graph, bkey);
        assert!(bubble.contains(centroid, &graph));
        
        // A point far outside the graph should not be inside
        assert!(!bubble.contains(Vec2::new(1000.0, 1000.0), &graph));
    }
}
