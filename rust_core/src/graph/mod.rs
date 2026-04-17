pub mod bubble;
pub mod edge;
pub mod point;
pub mod vertex;

use bubble::{Bubble, BubbleKey, BubbleStyle};
use edge::{Edge, EdgeKey};
use slotmap::SlotMap;
use vertex::{Vertex, VertexKey};
use crate::graphics::colors;

use crate::graph::point::Point;

pub struct Graph {
    pub vertecies: SlotMap<VertexKey, Vertex>,
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
            vertecies: SlotMap::with_key(),
            bubbles: SlotMap::with_key(),
        }
    }

    pub fn init(&mut self) {
        self.vertecies.clear();
        self.bubbles.clear();

        let vkey1 = self.vertecies.insert(Vertex::new(Point::new(0.0, -50.0)));
        let vkey2 = self.vertecies.insert(Vertex::new(Point::new(0.0, 50.0)));
        let ekeys1 = vkey1.edge_keys();
        let ekeys2 = vkey2.edge_keys();

        // Give the control points an initial offset so they aren't flat lines
        self.get_edge_mut(ekeys1[0]).point.position.x = -50.0;
        self.get_edge_mut(ekeys2[0]).point.position.x = -50.0;
        
        self.get_edge_mut(ekeys1[1]).point.position.x = 0.0;
        self.get_edge_mut(ekeys2[2]).point.position.x = 0.0;
        
        self.get_edge_mut(ekeys1[2]).point.position.x = 50.0;
        self.get_edge_mut(ekeys2[1]).point.position.x = 50.0;

        self.twin(ekeys1[0], ekeys2[0]);
        self.twin(ekeys1[1], ekeys2[2]);
        self.twin(ekeys1[2], ekeys2[1]);

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

        self.vertecies.remove(ekey.vertex);
        self.vertecies.remove(twin.vertex);
    }

    /*
    When a wall gets too short, We change its orientation: moving it's
    edges from the bubble they are on to the ones opposite.

    Before:           After:
    \        /          \   /
     \      /            \ /
      ------              |
     /      \            / \
    /        \          /   \
    */
    pub fn slide(&mut self, ekey: EdgeKey) {
        let twin_ekey = self.get_edge(ekey).twin;

        let a = self.half_slide(ekey);
        let b = self.half_slide(twin_ekey);

        self.twin(a, b);

        let next_a = self.next_on_bubble(a);
        let bubble_a = self.get_edge(next_a).bubble;
        self.rebubble(bubble_a, next_a);

        let next_b = self.next_on_bubble(b);
        let bubble_b = self.get_edge(next_b).bubble;
        self.rebubble(bubble_b, next_b);
    }

    // Removes the previous edge from the bubble and returns it
    fn half_slide(&mut self, ekey: EdgeKey) -> EdgeKey {
        let prev_on_bubble = self.prev_on_bubble(ekey);
        let new_twin = self.get_edge(prev_on_bubble).twin;

        self.twin(ekey, new_twin);

        let b = self.get_edge(ekey).bubble;
        self.rebubble(b, ekey);

        prev_on_bubble
    }

    

    fn twin(&mut self, a: EdgeKey, b: EdgeKey) {
        self.get_edge_mut(a).twin = b;
        self.get_edge_mut(b).twin = a;
    }

    fn rebubble(&mut self, bkey: BubbleKey, ekey: EdgeKey) {
        self.bubbles[bkey].edges.clear();

        let mut next_edge = ekey;
        loop {
            self.get_edge_mut(next_edge).bubble = bkey;
            self.bubbles[bkey].edges.push(next_edge);
            next_edge = self.next_on_bubble(next_edge);
            if next_edge == ekey {
                break;
            }
        }
    }

    fn next_on_bubble(&self, key: EdgeKey) -> EdgeKey {
        self.get_edge(key.next_on_vertex()).twin
    }

    fn prev_on_bubble(&self, key: EdgeKey) -> EdgeKey {
        self.get_edge(key).twin.next_on_vertex()
    }

    pub fn get_edge(&self, key: EdgeKey) -> &Edge {
        &self.vertecies[key.vertex].edges[key.offset as usize]
    }

    pub fn get_edge_and_vertex(&self, key: EdgeKey) -> (&Edge, &Vertex) {
        let vertex = &self.vertecies[key.vertex];
        (&vertex.edges[key.offset as usize], vertex)
    }

    fn get_edge_mut(&mut self, key: EdgeKey) -> &mut Edge {
        &mut self.vertecies[key.vertex].edges[key.offset as usize]
    }

    pub fn print_graph(&self) {
        for (vertex_key, vertex) in &self.vertecies {
            println!("Vertex {:?}: ({:?})", vertex_key, vertex.point);
            for edge in &vertex.edges {
                let twin_vertex = &self.vertecies[edge.twin.vertex];
                println!("  Edge to Vertex {:?}: ({:?})", edge.twin.vertex, twin_vertex.point);
            }
        }
    }

    pub fn get_open_air_vertices(&self) -> Vec<VertexKey> {
        let open_air_key = self.bubbles.iter()
            .find(|(_, b)| b.style == BubbleStyle::OpenAir)
            .map(|(k, _)| k);
        
        if let Some(key) = open_air_key {
            self.bubbles[key].edges.iter().map(|e| e.vertex).collect()
        } else {
            Vec::new()
        }
    }

    pub fn spawn(&mut self, _vkey: VertexKey, _style: BubbleStyle) {
        // To be implemented
    }
}
