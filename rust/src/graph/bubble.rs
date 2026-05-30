use super::edge::EdgeKey;
use crate::graph::Graph;
use crate::graphics::bubble;
use macroquad::math::Vec2;
use macroquad::prelude::Color;
use slotmap::{basic::Iter, basic::IterMut, new_key_type, SlotMap};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BubbleStyle {
    /// A standard colored bubble. Multiple colored bubbles can merge to increase their `size`.
    Colored { size: i32, color: Color },
    /// The unique player-controlled bubble. It has a limited number of swaps before game over.
    Swappable { swaps_left: i32, area: f32 },
    /// The unique infinite bubble representing the outside world.
    OpenAir,
    /// A temporary state used during the popping animation before the bubble is reaped.
    Invisible { size: i32 },
}

impl BubbleStyle {
    pub fn swappable(swaps_left: i32) -> Self {
        BubbleStyle::Swappable {
            swaps_left,
            area: 3000.0,
        }
    }

    pub fn colored(color: Color) -> Self {
        BubbleStyle::Colored { size: 1, color }
    }

    pub fn is_poppable(&self) -> bool {
        match self {
            BubbleStyle::Colored { size, .. } => *size >= 5,
            _ => false,
        }
    }

    pub fn get_target_area(&self) -> f32 {
        match self {
            BubbleStyle::Colored { size, .. } | BubbleStyle::Invisible { size } => {
                3000.0 * (*size as f32).sqrt()
            }
            BubbleStyle::Swappable { area, .. } => *area,
            BubbleStyle::OpenAir => 0.0,
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

    pub fn contains(&self, point: Vec2, graph: &Graph) -> bool {
        let points = bubble::get_points_for_bubble(graph, self);
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

    /// Returns true if the bubble's boundary self-intersects when treated as a vertex-to-vertex polygon.
    /// Uses the Shamos-Hoey algorithm for finding segment intersections (O(N log N)).
    pub fn has_self_intersection(&self, graph: &Graph) -> bool {
        let n = self.edges.len();
        if n < 3 {
            return false;
        }

        let vertices: Vec<Vec2> = self
            .edges
            .iter()
            .map(|&ekey| graph.vertices[ekey.vertex].point.position)
            .collect();

        #[derive(Debug, Clone, Copy)]
        struct Segment {
            id: usize,
            p1: Vec2,
            p2: Vec2,
        }

        let mut segments = Vec::with_capacity(n);
        for i in 0..n {
            let p1 = vertices[i];
            let p2 = vertices[(i + 1) % n];
            let (left, right) = if p1.x < p2.x || (p1.x == p2.x && p1.y < p2.y) {
                (p1, p2)
            } else {
                (p2, p1)
            };
            segments.push(Segment {
                id: i,
                p1: left,
                p2: right,
            });
        }

        #[derive(Debug, Clone, Copy)]
        enum EventType {
            Start,
            End,
        }

        #[derive(Debug, Clone, Copy)]
        struct Event {
            x: f32,
            y: f32,
            event_type: EventType,
            segment_idx: usize,
        }

        let mut events = Vec::with_capacity(2 * n);
        for (i, seg) in segments.iter().enumerate() {
            events.push(Event {
                x: seg.p1.x,
                y: seg.p1.y,
                event_type: EventType::Start,
                segment_idx: i,
            });
            events.push(Event {
                x: seg.p2.x,
                y: seg.p2.y,
                event_type: EventType::End,
                segment_idx: i,
            });
        }

        events.sort_by(|a, b| {
            a.x.partial_cmp(&b.x)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.y.partial_cmp(&b.y).unwrap_or(std::cmp::Ordering::Equal))
                .then_with(|| match (a.event_type, b.event_type) {
                    (EventType::Start, EventType::End) => std::cmp::Ordering::Less,
                    (EventType::End, EventType::Start) => std::cmp::Ordering::Greater,
                    _ => std::cmp::Ordering::Equal,
                })
        });

        fn ccw(a: Vec2, b: Vec2, c: Vec2) -> f32 {
            (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
        }

        fn on_segment(p: Vec2, q: Vec2, r: Vec2) -> bool {
            r.x >= p.x.min(q.x) - 1e-5
                && r.x <= p.x.max(q.x) + 1e-5
                && r.y >= p.y.min(q.y) - 1e-5
                && r.y <= p.y.max(q.y) + 1e-5
        }

        fn standard_intersection(a: Vec2, b: Vec2, c: Vec2, d: Vec2) -> bool {
            let ccw1 = ccw(a, b, c);
            let ccw2 = ccw(a, b, d);
            let ccw3 = ccw(c, d, a);
            let ccw4 = ccw(c, d, b);

            if ((ccw1 > 0.0 && ccw2 < 0.0) || (ccw1 < 0.0 && ccw2 > 0.0))
                && ((ccw3 > 0.0 && ccw4 < 0.0) || (ccw3 < 0.0 && ccw4 > 0.0))
            {
                return true;
            }

            if ccw1.abs() < 1e-5 && on_segment(a, b, c) {
                return true;
            }
            if ccw2.abs() < 1e-5 && on_segment(a, b, d) {
                return true;
            }
            if ccw3.abs() < 1e-5 && on_segment(c, d, a) {
                return true;
            }
            if ccw4.abs() < 1e-5 && on_segment(c, d, b) {
                return true;
            }

            false
        }

        fn segments_intersect(s1: &Segment, s2: &Segment, n: usize) -> bool {
            let diff = (s1.id as isize - s2.id as isize).abs();
            let is_adjacent = diff == 1 || diff == (n as isize - 1);

            if is_adjacent {
                let (shared, other1, other2) = if (s1.p1 - s2.p1).length_squared() < 1e-8 {
                    (s1.p1, s1.p2, s2.p2)
                } else if (s1.p1 - s2.p2).length_squared() < 1e-8 {
                    (s1.p1, s1.p2, s2.p1)
                } else if (s1.p2 - s2.p1).length_squared() < 1e-8 {
                    (s1.p2, s1.p1, s2.p2)
                } else if (s1.p2 - s2.p2).length_squared() < 1e-8 {
                    (s1.p2, s1.p1, s2.p1)
                } else {
                    return standard_intersection(s1.p1, s1.p2, s2.p1, s2.p2);
                };

                let ccw_val = ccw(other1, shared, other2);
                if ccw_val.abs() < 1e-5 {
                    let d1 = other1 - shared;
                    let d2 = other2 - shared;
                    if d1.dot(d2) > 1e-5 {
                        return true;
                    }
                }
                return false;
            }

            standard_intersection(s1.p1, s1.p2, s2.p1, s2.p2)
        }

        fn get_y_at_x(seg: &Segment, x: f32) -> f32 {
            if (seg.p2.x - seg.p1.x).abs() < 1e-6 {
                seg.p1.y.min(seg.p2.y)
            } else {
                let t = (x - seg.p1.x) / (seg.p2.x - seg.p1.x);
                seg.p1.y + t * (seg.p2.y - seg.p1.y)
            }
        }

        let mut active: Vec<Segment> = Vec::new();

        for event in events {
            let segment = segments[event.segment_idx];
            match event.event_type {
                EventType::Start => {
                    let x = event.x;
                    let mut insert_idx = 0;
                    while insert_idx < active.len() {
                        let y_active = get_y_at_x(&active[insert_idx], x);
                        if event.y > y_active + 1e-5 {
                            insert_idx += 1;
                        } else {
                            break;
                        }
                    }
                    active.insert(insert_idx, segment);

                    if insert_idx > 0 && segments_intersect(&active[insert_idx - 1], &segment, n) {
                        return true;
                    }
                    if insert_idx + 1 < active.len()
                        && segments_intersect(&active[insert_idx + 1], &segment, n)
                    {
                        return true;
                    }
                }
                EventType::End => {
                    if let Some(idx) = active.iter().position(|s| s.id == event.segment_idx) {
                        if idx > 0
                            && idx + 1 < active.len()
                            && segments_intersect(&active[idx - 1], &active[idx + 1], n)
                        {
                            return true;
                        }
                        active.remove(idx);
                    }
                }
            }
        }

        false
    }

    pub fn get_pressure(&self) -> f32 {
        if matches!(self.style, BubbleStyle::OpenAir) {
            return 1.0;
        }
        self.style.get_target_area() / self.area.max(10.0)
    }
}

#[derive(Default)]
pub struct BubbleSet {
    pub inner: SlotMap<BubbleKey, Bubble>,
}

impl BubbleSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_swappable(&self) -> Option<BubbleKey> {
        self.inner
            .iter()
            .find_map(|(k, b)| matches!(b.style, BubbleStyle::Swappable { .. }).then_some(k))
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
    type IntoIter = Iter<'a, BubbleKey, Bubble>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

impl<'a> IntoIterator for &'a mut BubbleSet {
    type Item = (BubbleKey, &'a mut Bubble);
    type IntoIter = IterMut<'a, BubbleKey, Bubble>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphics::colors;
    use crate::physics;

    #[test]
    fn test_bubble_contains() {
        let mut graph = Graph::new(
            BubbleStyle::Colored {
                size: 1,
                color: colors::TURQUOISE,
            },
            BubbleStyle::Colored {
                size: 1,
                color: colors::ROSE,
            },
        );

        // Run physics a few frames to expand the bubbles from their initial degenerate state
        for _ in 0..10 {
            physics::advance_frame(&mut graph);
        }

        let bkey = graph.bubbles.keys().next().unwrap();
        let bubble = &graph.bubbles[bkey];

        // The centroid of a valid bubble should be inside it
        let centroid = graph.bubbles[bkey].centroid;
        assert!(bubble.contains(centroid, &graph));

        // A point far outside the graph should not be inside
        assert!(!bubble.contains(Vec2::new(1000.0, 1000.0), &graph));
    }

    #[test]
    fn test_has_self_intersection() {
        let mut graph = Graph::new(
            BubbleStyle::colored(colors::TURQUOISE),
            BubbleStyle::colored(colors::ROSE),
        );

        // Digons initially return false early
        for bubble in graph.bubbles.values() {
            assert!(!bubble.has_self_intersection(&graph));
        }

        // Spawn a bubble to get bubbles with >= 3 vertices
        let vkeys: Vec<_> = graph.vertices.keys().collect();
        graph.spawn(vkeys[0], BubbleStyle::colored(colors::GREEN));

        // Find the newly spawned bubble (it's green)
        let green_bkey = graph.bubbles.iter()
            .find(|(_, b)| matches!(b.style, BubbleStyle::Colored { color, .. } if color == colors::GREEN))
            .map(|(k, _)| k)
            .unwrap();

        let bubble = &graph.bubbles[green_bkey];
        assert_eq!(bubble.edges.len(), 3); // It's a triangle!
        assert!(!bubble.has_self_intersection(&graph)); // A healthy triangle has no self-intersection

        // Now let's artificially cross one of the edges to make a self-intersection.
        // A healthy triangle: vertices A, B, C.
        // Let's find their keys
        let ekeys = &bubble.edges;
        let v0 = ekeys[0].vertex;
        let v1 = ekeys[1].vertex;
        let v2 = ekeys[2].vertex;

        // Let's set positions so that the edges cross
        // A = (0, 0), B = (10, 10), C = (5, 5) -> wait, if C lies on AB, that's collinear intersection
        // Or A = (0, 0), B = (10, 0), C = (5, 10) -> healthy
        // Let's make a self-intersecting quadrilateral by spawning one more time
        // or we can just set the triangle vertices to be collinear: A = (0,0), B = (10,0), C = (5,0)
        // Let's try:
        graph.vertices[v0].point.position = Vec2::new(0.0, 0.0);
        graph.vertices[v1].point.position = Vec2::new(10.0, 0.0);
        graph.vertices[v2].point.position = Vec2::new(5.0, 0.0);

        // C = (5, 0) is collinear and lies strictly inside segment AB = (0,0) -> (10,0).
        // This is a self-intersecting degenerate polygon (the boundary folds back on itself).
        assert!(bubble.has_self_intersection(&graph));
    }
}
