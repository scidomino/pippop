use super::edge::EdgeKey;
use macroquad::math::Vec2;
use macroquad::prelude::Color;
use slotmap::{new_key_type, SlotMap};
use std::ops::{Deref, DerefMut};

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
    Popping {
        size: i32,
        color: Color,
        timer: f32,
    },
}

impl BubbleStyle {
    pub fn is_poppable(&self) -> bool {
        match self {
            BubbleStyle::Standard { size, .. } => *size >= 5,
            _ => false,
        }
    }

    pub fn merge(&self, other: &BubbleStyle) -> BubbleStyle {
        match (self, other) {
            (BubbleStyle::OpenAir, _) | (_, BubbleStyle::OpenAir) => BubbleStyle::OpenAir,
            (BubbleStyle::Standard { size: s1, color }, BubbleStyle::Standard { size: s2, .. }) => {
                BubbleStyle::Standard {
                    size: s1 + s2,
                    color: *color,
                }
            }
            _ => self.clone(),
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
            BubbleStyle::Popping { size, timer, .. } => {
                let target = 3000.0 * (*size as f32).sqrt();
                if *timer <= 0.0 {
                    0.0
                } else {
                    target
                }
            }
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
    /// The cached 2D area of the bubble in world space.
    /// Updated during `Graph::update_cache` and used for physics pressure calculations.
    pub area: f32,
    /// The cached geometric center (center of mass) of the bubble.
    /// Updated during `Graph::update_cache` and used for UI labels and physics targets.
    pub centroid: Vec2,
}

impl Bubble {
    pub fn new(style: BubbleStyle) -> Self {
        Bubble {
            style,
            edges: Vec::new(),
            area: 0.0,
            centroid: Vec2::ZERO,
        }
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

    pub fn get_pressure(&self) -> f32 {
        if matches!(self.style, BubbleStyle::OpenAir) {
            return 1.0;
        }
        self.style.get_target_area() / self.area.max(100.0)
    }
}

pub struct BubbleSet {
    pub inner: SlotMap<BubbleKey, Bubble>,
}

impl BubbleSet {
    pub fn new() -> Self {
        Self {
            inner: SlotMap::with_key(),
        }
    }

    pub fn get_player_bubble(&self) -> Option<BubbleKey> {
        self.inner
            .iter()
            .find_map(|(k, b)| matches!(b.style, BubbleStyle::Player).then_some(k))
    }

    pub fn get_open_air(&self) -> BubbleKey {
        self.inner
            .iter()
            .find_map(|(k, b)| matches!(b.style, BubbleStyle::OpenAir).then_some(k))
            .expect("Graph must contain an open air bubble")
    }
}

impl Deref for BubbleSet {
    type Target = SlotMap<BubbleKey, Bubble>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for BubbleSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'a> IntoIterator for &'a BubbleSet {
    type Item = (BubbleKey, &'a Bubble);
    type IntoIter = slotmap::basic::Iter<'a, BubbleKey, Bubble>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<'a> IntoIterator for &'a mut BubbleSet {
    type Item = (BubbleKey, &'a mut Bubble);
    type IntoIter = slotmap::basic::IterMut<'a, BubbleKey, Bubble>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter_mut()
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
        let centroid = graph.bubbles[bkey].centroid;
        assert!(bubble.contains(centroid, &graph));

        // A point far outside the graph should not be inside
        assert!(!bubble.contains(Vec2::new(1000.0, 1000.0), &graph));
    }
}
