use super::edge::EdgeKey;
use macroquad::math::Vec2;
use macroquad::prelude::Color;
use slotmap::new_key_type;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BubbleStyle {
    Standard {
        size: i32,
        color: Color,
    },
    Player,
    OpenAir,
    Waiting {
        start_area: f32,
        end_area: f32,
        progress: f32,
    },
}

impl BubbleStyle {
    pub fn merge(&self, other: &BubbleStyle) -> BubbleStyle {
        match (self, other) {
            (BubbleStyle::OpenAir, _) | (_, BubbleStyle::OpenAir) => BubbleStyle::OpenAir,
            (BubbleStyle::Standard { size: s1, color }, BubbleStyle::Standard { size: s2, .. }) => {
                BubbleStyle::Standard {
                    size: s1 + s2,
                    color: *color,
                }
            }
            _ => unreachable!("merge should only be called with Standard or OpenAir styles"),
        }
    }

    pub fn get_target_area(&self) -> f32 {
        match self {
            BubbleStyle::Standard { size, .. } => 3000.0 * (*size as f32).sqrt(),
            BubbleStyle::Player => 3000.0,
            BubbleStyle::OpenAir => 0.0,
            BubbleStyle::Waiting {
                start_area,
                end_area,
                progress,
            } => start_area + (end_area - start_area) * progress,
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
        if points.len() < 3 {
            return false;
        }

        points
            .iter()
            .zip(points.iter().cycle().skip(1))
            .filter(|(p1, p2)| {
                ((p1.y > point.y) != (p2.y > point.y))
                    && (point.x < (p2.x - p1.x) * (point.y - p1.y) / (p2.y - p1.y) + p1.x)
            })
            .count()
            % 2
            != 0
    }

    pub fn get_pressure(&self, area: f32) -> f32 {
        if matches!(self.style, BubbleStyle::OpenAir) {
            return 1.0;
        }
        self.style.get_target_area() / area.max(100.0)
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
        graph.init(
            BubbleStyle::Standard {
                size: 1,
                color: crate::graphics::colors::TURQUOISE,
            },
            BubbleStyle::Standard {
                size: 1,
                color: crate::graphics::colors::ROSE,
            },
        );

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
