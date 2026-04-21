pub mod bubble;
pub mod edge;
pub mod point;
pub mod vertex;

use bubble::{Bubble, BubbleKey, BubbleStyle};
use edge::{Edge, EdgeKey};
use macroquad::math::Vec2;
use slotmap::SlotMap;
use vertex::{Vertex, VertexKey};

use crate::graph::point::Point;

/// A Half-Edge data structure representing the planar graph of bubbles.
///
/// This graph enforces a trivalent (honeycomb) topology, meaning every
/// vertex is connected to exactly three edges. The graph is planar and always
/// connected, with a single "open air" bubble representing the outside.
///
/// The graph supports all valid planar topologies, including "degenerate" cases
/// such as monogons (bubbles with only one vertex and a self-looped edge) and
/// digons (bubbles with only two vertices).
pub struct Graph {
    pub vertices: SlotMap<VertexKey, Vertex>,
    pub bubbles: SlotMap<BubbleKey, Bubble>,
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            vertices: SlotMap::with_key(),
            bubbles: SlotMap::with_key(),
        }
    }

    pub fn init(&mut self, s1: BubbleStyle, s2: BubbleStyle) {
        self.vertices.clear();
        self.bubbles.clear();

        let v1 = self.vertices.insert(Vertex::new(Point::new(0.0, -50.0)));
        let v2 = self.vertices.insert(Vertex::new(Point::new(0.0, 50.0)));
        let e1 = v1.edge_keys();
        let e2 = v2.edge_keys();

        self.link_twins(e1[0], e2[0]);
        self.link_twins(e1[1], e2[2]);
        self.link_twins(e1[2], e2[1]);

        for (s, e) in [(s1, e1[0]), (s2, e1[1]), (BubbleStyle::OpenAir, e1[2])] {
            let b = self.bubbles.insert(Bubble::new(s));
            self.rebubble(b, e);
        }
    }

    /// Removes an edge and its twin by removing their vertices and merging the two bubbles they separate.
    ///
    /// Special cases:
    /// - If the edge has the same bubble on both sides, it cannot be removed.
    ///
    /// ```text
    /// Before:           After:
    /// \        /
    ///  \      /
    ///    -  -              \      /
    ///  /      \            /      \
    /// /        \
    /// ```
    pub fn remove_edge(&mut self, ekey: EdgeKey) {
        let tkey = self.get_edge(ekey).twin;
        let b_top = self.get_edge(ekey).bubble;
        let b_bottom = self.get_edge(tkey).bubble;

        if b_top == b_bottom {
            // Can't remove edge with same bubble on both sides
            // as this would create a disconnected graph.
            log::info!("remove_edge: skipped (same bubble on both sides)");
            return;
        }

        let ekey_next = ekey.next_on_vertex();
        let ekey_prev = ekey.prev_on_vertex();
        let tkey_next = tkey.next_on_vertex();
        let tkey_prev = tkey.prev_on_vertex();

        let ekey_next_twin = self.get_edge(ekey_next).twin;
        let ekey_prev_twin = self.get_edge(ekey_prev).twin;
        let tkey_next_twin = self.get_edge(tkey_next).twin;
        let tkey_prev_twin = self.get_edge(tkey_prev).twin;

        let b_right = self.get_edge(ekey_next).bubble;
        let b_left = self.get_edge(tkey_next).bubble;

        // Merge b_bottom into b_top
        let bottom_style = self.bubbles[b_bottom].style;
        let mut_b_top = &mut self.bubbles[b_top];
        mut_b_top.style = mut_b_top.style.merge(&bottom_style);

        if ekey_next == tkey_prev_twin {
            // the top bubble only has two edges so b_left == b_right.
            log::info!("remove_edge: top bubble only has two edges");

            self.link_twins(ekey_prev_twin, tkey_next_twin);

            self.rebubble(b_top, tkey_next_twin);
            self.rebubble(b_right, ekey_prev_twin);
        } else if ekey_prev == tkey_next_twin {
            // the bottom bubble only has two edges so b_left == b_right.
            log::info!("remove_edge: bottom bubble only has two edges");

            self.link_twins(ekey_next_twin, tkey_prev_twin);

            self.rebubble(b_top, tkey_prev_twin);
            self.rebubble(b_right, ekey_next_twin);
        } else {
            log::info!("remove_edge: standard merge");
            self.link_twins(ekey_next_twin, ekey_prev_twin);
            self.link_twins(tkey_next_twin, tkey_prev_twin);

            self.rebubble(b_top, ekey_next_twin);
            self.rebubble(b_right, ekey_prev_twin);
            self.rebubble(b_left, tkey_prev_twin);
        }

        self.bubbles.remove(b_bottom);
        self.vertices.remove(ekey.vertex);
        self.vertices.remove(tkey.vertex);

        for degenerate in self.get_degenerate_edges(b_top) {
            log::info!("remove_edge: found degenerate edge {:?}", degenerate);
            self.slide(degenerate);
        }
    }

    /// When a wall gets too short, We change its orientation: moving its
    /// edges from the bubble they are on to the ones opposite.
    /// This is a topological T1 flip.
    ///
    /// Before:           After:
    /// \        /          \   /
    ///  \      /            \ /
    ///    -  -               ¦
    ///  /      \            / \
    /// /        \          /   \
    pub fn slide(&mut self, ekey: EdgeKey) {
        let tkey = self.get_edge(ekey).twin;

        let b_top = self.get_edge(ekey).bubble;
        let b_bottom = self.get_edge(tkey).bubble;

        // note: it is possible for b_top == b_bottom if a bubble borders itself.
        // We still want to slide in this case.

        let ekey_prev = ekey.prev_on_vertex();
        let tkey_prev = tkey.prev_on_vertex();

        let ekey_prev_tkey = self.get_edge(ekey_prev).twin;
        let tkey_prev_tkey = self.get_edge(tkey_prev).twin;

        let b_right = self.get_edge(ekey_prev_tkey).bubble;
        let b_left = self.get_edge(tkey_prev_tkey).bubble;

        if b_right == b_left {
            // Don't slide if it would cause a bubble to border itself
            return;
        }

        let e_point = self.get_edge(ekey).point;
        let t_point = self.get_edge(tkey).point;
        let eprev_point = self.get_edge(ekey_prev).point;
        let tprev_point = self.get_edge(tkey_prev).point;

        // Perform topological flip by rotating the external connections
        self.link_twins(ekey, tkey_prev_tkey);
        self.link_twins(tkey, ekey_prev_tkey);
        self.link_twins(tkey_prev, ekey_prev);

        // Update control points
        self.get_edge_mut(ekey).point = tprev_point;
        self.get_edge_mut(tkey).point = eprev_point;
        self.get_edge_mut(tkey_prev).point = e_point;
        self.get_edge_mut(ekey_prev).point = t_point;

        // Rebuild the boundaries of the 4 affected bubbles
        self.rebubble(b_top, ekey);
        self.rebubble(b_bottom, tkey);
        self.rebubble(b_left, ekey_prev);
        self.rebubble(b_right, tkey_prev);
    }

    fn link_twins(&mut self, a: EdgeKey, b: EdgeKey) {
        self.get_edge_mut(a).twin = b;
        self.get_edge_mut(b).twin = a;
    }

    pub fn rebubble(&mut self, bkey: BubbleKey, ekey: EdgeKey) {
        self.bubbles[bkey].edges.clear();

        let mut next_edge = ekey;
        for _ in 0..1000 {
            self.get_edge_mut(next_edge).bubble = bkey;
            self.bubbles[bkey].edges.push(next_edge);
            next_edge = self.next_on_bubble(next_edge);
            if next_edge == ekey {
                return;
            }
        }

        panic!(
            "Infinite loop detected in rebubble for bubble {:?}. Started at edge {:?}",
            bkey, ekey
        );
    }

    pub fn get_player_bubble(&self) -> Option<BubbleKey> {
        self.bubbles
            .iter()
            .find_map(|(k, b)| matches!(b.style, BubbleStyle::Player).then_some(k))
    }

    pub fn get_closest_otter_swappable(&self, point: Vec2) -> Option<EdgeKey> {
        let player_bkey = self.get_player_bubble()?;

        self.bubbles[player_bkey]
            .edges
            .iter()
            .filter_map(|&ekey| {
                let twin_ekey = self.get_edge(ekey).twin;
                let bkey = self.get_edge(twin_ekey).bubble;

                if let BubbleStyle::Standard { .. } = self.bubbles[bkey].style {
                    let centroid = crate::graphics::geometry::calculate_centroid(self, bkey);
                    Some((twin_ekey, centroid.distance_squared(point)))
                } else {
                    None
                }
            })
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(ekey, _)| ekey)
    }

    // next edge on the same bubble in clockwise order
    pub fn next_on_bubble(&self, key: EdgeKey) -> EdgeKey {
        self.get_edge(key).twin.prev_on_vertex()
    }

    pub fn get_edge(&self, key: EdgeKey) -> &Edge {
        &self.vertices[key.vertex].edges[key.offset as usize]
    }

    pub fn get_bezier(&self, ekey: EdgeKey) -> crate::graphics::geometry::Bezier {
        let (edge, vertex) = self.get_edge_and_vertex(ekey);
        let (twin, twin_vertex) = self.get_edge_and_vertex(edge.twin);
        crate::graphics::geometry::Bezier::from_points(
            vertex.point.position,
            edge.point.position,
            twin.point.position,
            twin_vertex.point.position,
        )
    }

    pub fn get_edge_and_vertex(&self, key: EdgeKey) -> (&Edge, &Vertex) {
        let vertex = &self.vertices[key.vertex];
        (&vertex.edges[key.offset as usize], vertex)
    }

    fn get_edge_mut(&mut self, key: EdgeKey) -> &mut Edge {
        &mut self.vertices[key.vertex].edges[key.offset as usize]
    }

    pub fn print_graph(&self) {
        for (vkey, vertex) in &self.vertices {
            println!("Vertex {:?}: ({:?})", vkey, vertex.point);
            for edge in &vertex.edges {
                let twin_vertex = &self.vertices[edge.twin.vertex];
                println!(
                    "  Edge to Vertex {:?}: ({:?})",
                    edge.twin.vertex, twin_vertex.point
                );
            }
        }
    }

    pub fn get_open_air_vertices(&self) -> Vec<VertexKey> {
        self.bubbles
            .iter()
            .find(|(_, b)| matches!(b.style, BubbleStyle::OpenAir))
            .map(|(_, b)| b.edges.iter().map(|e| e.vertex).collect())
            .unwrap_or_default()
    }

    pub fn get_degenerate_edges(&self, bkey: BubbleKey) -> Vec<EdgeKey> {
        let mut degenerates = Vec::new();
        if let Some(bubble) = self.bubbles.get(bkey) {
            for &ekey in &bubble.edges {
                let twin_ekey = self.get_edge(ekey).twin;
                if self.get_edge(twin_ekey).bubble == bkey {
                    // Prevent returning both the edge and its twin
                    if !degenerates.contains(&ekey) && !degenerates.contains(&twin_ekey) {
                        degenerates.push(ekey);
                    }
                }
            }
        }
        degenerates
    }

    /// Spawns a new bubble by "splitting" a vertex into three vertices.
    ///
    /// This operation inserts two new vertices and creates a small triangular
    /// bubble between the original vertex and the two new ones.
    pub fn spawn(&mut self, vkey: VertexKey, style: BubbleStyle) {
        let vertex: Vertex = self.vertices[vkey];
        let ekeys: [EdgeKey; 3] = vkey.edge_keys();
        let twin_ekeys = vertex.edges.map(|e| e.twin);
        let bkeys = vertex.edges.map(|e| e.bubble);

        let vpoint = vertex.point.position;
        let epoint1 = vertex.edges[0].point.position;
        let epoint2 = vertex.edges[1].point.position;

        let new_vkey1 = self
            .vertices
            .insert(Vertex::new(Point::from_vec2((vpoint + epoint1) / 2.0)));
        let new_vkey2 = self
            .vertices
            .insert(Vertex::new(Point::from_vec2((vpoint + epoint2) / 2.0)));

        let new_ekeys1 = new_vkey1.edge_keys();
        let new_ekeys2 = new_vkey2.edge_keys();

        // Connect the new vertices to each other.
        self.link_twins(new_ekeys1[0], new_ekeys2[0]);

        self.link_twins(new_ekeys1[1], ekeys[0]);
        self.link_twins(new_ekeys1[2], twin_ekeys[0]);

        self.link_twins(new_ekeys2[1], twin_ekeys[1]);
        self.link_twins(new_ekeys2[2], ekeys[1]);

        let new_bkey = self.bubbles.insert(Bubble::new(style));
        self.rebubble(new_bkey, ekeys[0]);
        self.rebubble(bkeys[1], ekeys[1]);
        self.rebubble(bkeys[2], ekeys[2]);
        self.rebubble(bkeys[0], new_ekeys2[0]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_player_bubble() {
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

        // Initially no player bubble in init()
        assert_eq!(graph.get_player_bubble(), None);

        // Set first bubble to Player
        let bkey = graph.bubbles.keys().next().unwrap();
        graph.bubbles[bkey].style = BubbleStyle::Player;

        assert_eq!(graph.get_player_bubble(), Some(bkey));
    }

    #[test]
    fn test_get_closest_otter_swappable() {
        let mut graph = Graph::new();
        graph.init(
            BubbleStyle::Player,
            BubbleStyle::Standard {
                size: 1,
                color: crate::graphics::colors::TURQUOISE,
            },
        );

        // Assign b1 as player and b2 as a standard bubble.
        let b2 = graph.bubbles.keys().nth(1).unwrap();

        // Find a point near b2
        let centroid2 = crate::graphics::geometry::calculate_centroid(&graph, b2);

        let closest = graph.get_closest_otter_swappable(centroid2);
        assert!(closest.is_some());

        let ekey = closest.unwrap();
        // The edge should belong to b2, since it is the swappable twin of the player's edge
        assert_eq!(graph.get_edge(ekey).bubble, b2);
    }

    #[test]
    fn test_rebubble_reorders_edges() {
        let mut graph = Graph::new();
        graph.init(
            BubbleStyle::Player,
            BubbleStyle::Standard {
                size: 1,
                color: crate::graphics::colors::TURQUOISE,
            },
        );

        let bkey = graph.bubbles.keys().next().unwrap();
        let original_edges = graph.bubbles[bkey].edges.clone();
        assert!(original_edges.len() >= 2);

        // Rebubble starting from the second edge
        let new_start = original_edges[1];
        graph.rebubble(bkey, new_start);

        let new_edges = &graph.bubbles[bkey].edges;
        assert_eq!(new_edges[0], new_start);
        assert_eq!(new_edges.len(), original_edges.len());

        // Ensure it's a cyclic shift
        for i in 0..new_edges.len() {
            assert_eq!(new_edges[i], original_edges[(i + 1) % original_edges.len()]);
        }
    }

    #[test]
    fn test_bridge_creation_and_resolution() {
        let mut graph = Graph::new();
        // Create a large enough graph by spawning a few times
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

        let oa_key = graph
            .bubbles
            .iter()
            .find(|(_, b)| matches!(b.style, BubbleStyle::OpenAir))
            .unwrap()
            .0;

        // Spawn a couple times to get a non-trivial graph
        let mut vkeys: Vec<_> = graph.vertices.keys().collect();
        graph.spawn(
            vkeys[0],
            BubbleStyle::Standard {
                size: 1,
                color: crate::graphics::colors::GREEN,
            },
        );
        vkeys = graph.vertices.keys().collect();
        graph.spawn(
            vkeys[2],
            BubbleStyle::Standard {
                size: 1,
                color: crate::graphics::colors::YELLOW,
            },
        );

        // Find an edge that separates two distinct standard bubbles
        let mut target_ekey = None;
        for (vkey, vertex) in &graph.vertices {
            for (offset, edge) in vertex.edges.iter().enumerate() {
                let twin_bubble = graph.get_edge(edge.twin).bubble;
                if edge.bubble != oa_key && twin_bubble != oa_key && edge.bubble != twin_bubble {
                    target_ekey = Some(vkey.edge_key(offset as u8));
                    break;
                }
            }
            if target_ekey.is_some() {
                break;
            }
        }

        let ekey = target_ekey.expect("Could not find internal edge");

        // Make it a bridge by setting both sides to OpenAir
        graph.get_edge_mut(ekey).bubble = oa_key;
        graph.get_edge_mut(graph.get_edge(ekey).twin).bubble = oa_key;

        // Slide it!
        graph.slide(ekey);

        // Verify that after sliding, the edge does not have OpenAir on both sides
        let new_b1 = graph.get_edge(ekey).bubble;
        let new_b2 = graph.get_edge(graph.get_edge(ekey).twin).bubble;

        assert!(
            new_b1 != oa_key || new_b2 != oa_key,
            "Bridge was not resolved!"
        );
    }
}
