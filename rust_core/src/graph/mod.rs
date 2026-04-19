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
/// vertex is connected to exactly three edges. This naturally simulates
/// Plateau's Laws for foams, where bubble walls always meet at 120-degree angles.
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

    pub fn init(&mut self, style1: BubbleStyle, style2: BubbleStyle) {
        self.vertices.clear();
        self.bubbles.clear();

        let vkey1 = self.vertices.insert(Vertex::new(Point::new(0.0, -50.0)));
        let vkey2 = self.vertices.insert(Vertex::new(Point::new(0.0, 50.0)));
        let ekeys1 = vkey1.edge_keys();
        let ekeys2 = vkey2.edge_keys();

        self.link_twins(ekeys1[0], ekeys2[0]);
        self.link_twins(ekeys1[1], ekeys2[2]);
        self.link_twins(ekeys1[2], ekeys2[1]);

        let b1 = self.bubbles.insert(Bubble::new(style1));
        self.rebubble(b1, ekeys1[0]);
        let b2 = self.bubbles.insert(Bubble::new(style2));
        self.rebubble(b2, ekeys1[1]);
        let open_air = self.bubbles.insert(Bubble::new(BubbleStyle::OpenAir));
        self.rebubble(open_air, ekeys1[2]);
    }

    pub fn remove_edge(&mut self, ekey: EdgeKey) {
        let twin = self.get_edge(ekey).twin;
        let b1 = self.get_edge(ekey).bubble;
        let b2 = self.get_edge(twin).bubble;

        if b1 == b2 {
            // Cannot remove edge with same bubble on both sides
            return;
        }

        let e1_next = ekey.next_on_vertex();
        let e1_prev = ekey.prev_on_vertex();
        let e2_next = twin.next_on_vertex();
        let e2_prev = twin.prev_on_vertex();

        let t1_next = self.get_edge(e1_next).twin;
        let t1_prev = self.get_edge(e1_prev).twin;
        let t2_next = self.get_edge(e2_next).twin;
        let t2_prev = self.get_edge(e2_prev).twin;

        // Ensure we are not dealing with a multigraph where two vertices share >1 edge.
        // If v1 and v2 share another edge, then t1_next, t1_prev, t2_next, or t2_prev
        // will originate from the vertex we are about to delete.
        // In a proper foam simulation, this can occur if a bubble is a digon (2 edges).
        if t1_next.vertex == twin.vertex || t1_prev.vertex == twin.vertex ||
           t2_next.vertex == ekey.vertex || t2_prev.vertex == ekey.vertex {
            // Removing one edge of a digon creates a bridge (degenerate edge)
            // For now, we safely abort the merge to prevent graph corruption.
            return;
        }

        // Merge b2 into b1
        let b2_style = self.bubbles[b2].style.clone();
        let b1_bubble = &mut self.bubbles[b1];
        b1_bubble.style = b1_bubble.style.merge(&b2_style);
        
        // Find all edges of b2 and set them to b1
        for (_, vertex) in self.vertices.iter_mut() {
            for edge in vertex.edges.iter_mut() {
                if edge.bubble == b2 {
                    edge.bubble = b1;
                }
            }
        }
        self.bubbles.remove(b2);

        // Update control points for smoother transition
        let p_t1_next = self.get_edge(t1_next).point.position;
        let p_e1_prev = self.get_edge(e1_prev).point.position;
        
        let p_t1_prev = self.get_edge(t1_prev).point.position;
        let p_e1_next = self.get_edge(e1_next).point.position;
        
        let p_t2_next = self.get_edge(t2_next).point.position;
        let p_e2_prev = self.get_edge(e2_prev).point.position;
        
        let p_t2_prev = self.get_edge(t2_prev).point.position;
        let p_e2_next = self.get_edge(e2_next).point.position;

        self.get_edge_mut(t1_next).point.position = (p_t1_next + p_e1_prev) / 2.0;
        self.get_edge_mut(t1_prev).point.position = (p_t1_prev + p_e1_next) / 2.0;
        self.get_edge_mut(t2_next).point.position = (p_t2_next + p_e2_prev) / 2.0;
        self.get_edge_mut(t2_prev).point.position = (p_t2_prev + p_e2_next) / 2.0;

        // Bypass v1
        self.link_twins(t1_next, t1_prev);
        // Bypass v2
        self.link_twins(t2_next, t2_prev);

        // Get the new bubbles bounded by these edges (after b2 was merged)
        let b_t1_next = self.get_edge(t1_next).bubble;
        let b_t1_prev = self.get_edge(t1_prev).bubble;
        let b_t2_next = self.get_edge(t2_next).bubble;
        let b_t2_prev = self.get_edge(t2_prev).bubble;

        // Remove v1 and v2
        self.vertices.remove(ekey.vertex);
        self.vertices.remove(twin.vertex);

        // Rebuild edge lists for the affected bubbles
        self.rebubble(b_t1_next, t1_next);
        self.rebubble(b_t1_prev, t1_prev);
        if self.bubbles.contains_key(b_t2_next) {
            self.rebubble(b_t2_next, t2_next);
        }
        if self.bubbles.contains_key(b_t2_prev) {
            self.rebubble(b_t2_prev, t2_prev);
        }
    }

    /*
    When a wall gets too short, We change its orientation: moving its
    edges from the bubble they are on to the ones opposite.
    This is a topological T1 flip.

    Before:           After:
    \        /          \   /
     \      /            \ /
      ------              |
     /      \            / \
    /        \          /   \
    */
    pub fn slide(&mut self, ekey: EdgeKey) {
        let e = ekey;
        let t = self.get_edge(e).twin;

        let b_left = self.get_edge(e).bubble;
        let b_right = self.get_edge(t).bubble;

        // Safety: cannot slide an edge that has the same bubble on both sides (a bridge).
        if b_left == b_right {
            return;
        }

        let e1 = e.next_on_vertex();
        let e2 = e.prev_on_vertex();
        let t1 = t.next_on_vertex();
        let t2 = t.prev_on_vertex();

        let b_top = self.get_edge(e1).bubble;
        let b_bottom = self.get_edge(t1).bubble;

        // Safety: cannot slide if it would merge two different boundaries of the same bubble
        // or create other topological monstrosities.
        if b_top == b_bottom {
            return;
        }

        let old_e1_twin = self.get_edge(e1).twin;
        let old_e2_twin = self.get_edge(e2).twin;
        let old_t1_twin = self.get_edge(t1).twin;
        let old_t2_twin = self.get_edge(t2).twin;

        let p_e1 = self.get_edge(e1).point;
        let p_e2 = self.get_edge(e2).point;
        let p_t1 = self.get_edge(t1).point;
        let p_t2 = self.get_edge(t2).point;

        // Perform topological flip by rotating the external connections
        self.link_twins(e1, old_t2_twin);
        self.link_twins(t2, old_t1_twin);
        self.link_twins(t1, old_e2_twin);
        self.link_twins(e2, old_e1_twin);

        self.get_edge_mut(e1).point = p_t2;
        self.get_edge_mut(t2).point = p_t1;
        self.get_edge_mut(t1).point = p_e2;
        self.get_edge_mut(e2).point = p_e1;

        // Rebuild the boundaries of the 4 affected bubbles
        self.rebubble(b_left, e1);
        self.rebubble(b_right, t1);
        self.rebubble(b_top, t);
        self.rebubble(b_bottom, e);
    }

    fn link_twins(&mut self, a: EdgeKey, b: EdgeKey) {
        self.get_edge_mut(a).twin = b;
        self.get_edge_mut(b).twin = a;
    }

    fn rebubble(&mut self, bkey: BubbleKey, ekey: EdgeKey) {
        self.bubbles[bkey].edges.clear();

        let mut next_edge = ekey;
        let mut i = 0;
        loop {
            self.get_edge_mut(next_edge).bubble = bkey;
            self.bubbles[bkey].edges.push(next_edge);
            next_edge = self.next_on_bubble(next_edge);
            if next_edge == ekey {
                break;
            }
            i += 1;
            if i > 1000 {
                panic!("Infinite loop detected in rebubble for bubble {:?}. Started at edge {:?}", bkey, ekey);
            }
        }
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
    fn next_on_bubble(&self, key: EdgeKey) -> EdgeKey {
        self.get_edge(key).twin.prev_on_vertex()
    }

    pub fn get_edge(&self, key: EdgeKey) -> &Edge {
        &self.vertices[key.vertex].edges[key.offset as usize]
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

    pub fn spawn(&mut self, vkey: VertexKey, style: BubbleStyle) {
        let vertex: Vertex = self.vertices[vkey];
        let ekeys: [EdgeKey; 3] = vkey.edge_keys();
        let twin_ekeys = [
            vertex.edges[0].twin,
            vertex.edges[1].twin,
            vertex.edges[2].twin,
        ];
        let bkeys = [
            vertex.edges[0].bubble,
            vertex.edges[1].bubble,
            vertex.edges[2].bubble,
        ];

        let vpoint = vertex.point.position;
        let epoint1 = vertex.edges[0].point.position;
        let epoint2 = vertex.edges[1].point.position;

        let new_vkey1 = self.vertices.insert(Vertex::new(Point::from_vec2((vpoint + epoint1) / 2.0)));
        let new_vkey2 = self.vertices.insert(Vertex::new(Point::from_vec2((vpoint + epoint2) / 2.0)));

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
    use macroquad::math::Vec2;

    #[test]
    fn test_get_player_bubble() {
        let mut graph = Graph::new();
        graph.init(
            BubbleStyle::Standard { size: 1, max_size: 5, color: crate::graphics::colors::TURQUOISE },
            BubbleStyle::Standard { size: 1, max_size: 5, color: crate::graphics::colors::ROSE },
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
            BubbleStyle::Standard { size: 1, max_size: 5, color: crate::graphics::colors::TURQUOISE },
        );
        
        // Assign b1 as player and b2 as a standard bubble.
        let mut keys = graph.bubbles.keys();
        let b1 = keys.next().unwrap();
        let b2 = keys.next().unwrap();
        
        // Find a point near b2
        let centroid2 = crate::graphics::geometry::calculate_centroid(&graph, b2);
        
        let closest = graph.get_closest_otter_swappable(centroid2);
        assert!(closest.is_some());
        
        let ekey = closest.unwrap();
        // The edge should belong to b2, since it is the swappable twin of the player's edge
        assert_eq!(graph.get_edge(ekey).bubble, b2);
    }
}
