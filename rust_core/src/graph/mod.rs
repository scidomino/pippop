pub mod bubble;
pub mod edge;
pub mod point;
pub mod vertex;

use crate::graphics::colors;
use bubble::{Bubble, BubbleKey, BubbleStyle};
use edge::{Edge, EdgeKey};
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

    pub fn init(&mut self) {
        self.vertices.clear();
        self.bubbles.clear();

        let vkey1 = self.vertices.insert(Vertex::new(Point::new(0.0, -50.0)));
        let vkey2 = self.vertices.insert(Vertex::new(Point::new(0.0, 50.0)));
        let ekeys1 = vkey1.edge_keys();
        let ekeys2 = vkey2.edge_keys();

        self.link_twins(ekeys1[0], ekeys2[0]);
        self.link_twins(ekeys1[1], ekeys2[2]);
        self.link_twins(ekeys1[2], ekeys2[1]);

        let b1 = self.bubbles.insert(Bubble::new(BubbleStyle::Standard {
            size: 1,
            max_size: 5,
            color: colors::TURQUOISE,
        }));
        self.rebubble(b1, ekeys1[0]);
        let b2 = self.bubbles.insert(Bubble::new(BubbleStyle::Standard {
            size: 1,
            max_size: 5,
            color: colors::ROSE,
        }));
        self.rebubble(b2, ekeys1[1]);
        let open_air = self.bubbles.insert(Bubble::new(BubbleStyle::OpenAir));
        self.rebubble(open_air, ekeys1[2]);
    }

    pub fn remove_edge(&mut self, ekey: EdgeKey) {
        let (twin, bkey) = {
            let edge = self.get_edge(ekey);
            (edge.twin, edge.bubble)
        };
        let twin_bkey = self.get_edge(twin).bubble;
        if bkey == twin_bkey {
            // Cannot remove edge with same bubble on both sides
            return;
        }

        let retained = self.get_edge(ekey.prev_on_vertex()).twin;
        let twin_retained = self.get_edge(twin.prev_on_vertex()).twin;

        let retained_bkey = self.get_edge(retained).bubble;
        let twin_retaned_bkey = self.get_edge(twin_retained).bubble;

        self.half_slide(retained);
        self.half_slide(twin_retained);

        let b2 = self.bubbles[twin_retaned_bkey].clone();
        let b1 = &mut self.bubbles[retained_bkey];

        b1.merge(&b2);
        self.bubbles.remove(twin_retaned_bkey);
        self.rebubble(retained_bkey, retained);

        self.vertices.remove(ekey.vertex);
        self.vertices.remove(twin.vertex);
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

    // Re-links neighbors of an edge to bypass a vertex, effectively "straightening"
    // a 2-way junction. Used during bubble removal.
    fn half_slide(&mut self, ekey: EdgeKey) -> EdgeKey {
        // In CW traversal, the edge pointing to ekey's start is ekey.next_on_vertex().twin
        let prev_on_bubble = self.prev_on_bubble(ekey);
        let next_on_vertex = ekey.next_on_vertex();
        let other_neighbor_twin = self.get_edge(next_on_vertex).twin;

        self.link_twins(ekey, other_neighbor_twin);

        let b = self.get_edge(ekey).bubble;
        self.rebubble(b, ekey);

        prev_on_bubble
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

    // next edge on the same bubble in clockwise order
    fn next_on_bubble(&self, key: EdgeKey) -> EdgeKey {
        self.get_edge(key).twin.prev_on_vertex()
    }

    // previous edge on the same bubble in clockwise order
    fn prev_on_bubble(&self, key: EdgeKey) -> EdgeKey {
        self.get_edge(key.next_on_vertex()).twin
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
        let open_air_key = self
            .bubbles
            .iter()
            .find(|(_, b)| b.style == BubbleStyle::OpenAir)
            .map(|(k, _)| k);

        if let Some(key) = open_air_key {
            self.bubbles[key].edges.iter().map(|e| e.vertex).collect()
        } else {
            Vec::new()
        }
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

        let mid_point1 = Point::new((vpoint.x + epoint1.x) / 2.0, (vpoint.y + epoint1.y) / 2.0);
        let mid_point2 = Point::new((vpoint.x + epoint2.x) / 2.0, (vpoint.y + epoint2.y) / 2.0);

        let new_vkey1 = self.vertices.insert(Vertex::new(mid_point1));
        let new_vkey2 = self.vertices.insert(Vertex::new(mid_point2));

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
