pub mod bubble;
pub mod edge;
pub mod point;
pub mod vertex;

use bubble::{Bubble, BubbleKey};
use edge::{Edge, EdgeKey};
use slotmap::SlotMap;
use vertex::{Vertex, VertexKey};

use crate::graph::point::Point;

pub struct RelationManager {
    pub vertecies: SlotMap<VertexKey, Vertex>,
    pub bubbles: SlotMap<BubbleKey, Bubble>,
}

impl RelationManager {
    pub fn new() -> Self {
        RelationManager {
            vertecies: SlotMap::with_key(),
            bubbles: SlotMap::with_key(),
        }
    }

    pub fn init(&mut self) {
        self.vertecies.clear();
        self.bubbles.clear();

        let vkey1 = self.vertecies.insert(Vertex::new(Point::new(50.0, 0.0)));
        let vkey2 = self.vertecies.insert(Vertex::new(Point::new(50.0, 100.0)));
        let ekeys1 = vkey1.edge_keys();
        let ekeys2 = vkey2.edge_keys();

        self.twin(ekeys1[0], ekeys2[0]);
        self.twin(ekeys1[1], ekeys2[2]);
        self.twin(ekeys1[2], ekeys2[1]);

        let open_air = self.bubbles.insert(Bubble::new(true));
        self.rebubble(open_air, ekeys1[0]);
        let b1 = self.bubbles.insert(Bubble::new(false));
        self.rebubble(b1, ekeys1[1]);
        let b2 = self.bubbles.insert(Bubble::new(false));
        self.rebubble(b2, ekeys1[2]);
    }

    pub fn remove_edge(&mut self, ekey: EdgeKey) {
        let twin = self.get_edge(ekey).twin;

        let bkey = self.get_edge(ekey).bubble;
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

        return prev_on_bubble;
    }

    fn twin(&mut self, a: EdgeKey, b: EdgeKey) {
        self.get_edge(a).twin = b;
        self.get_edge(b).twin = a;
    }

    fn rebubble(&mut self, bkey: BubbleKey, ekey: EdgeKey) {
        self.bubbles[bkey].edges.clear();

        let mut next_edge = ekey;
        loop {
            self.get_edge(next_edge).bubble = bkey;
            self.bubbles[bkey].edges.push(next_edge);
            next_edge = self.next_on_bubble(next_edge);
            if next_edge == ekey {
                break;
            }
        }
    }

    // Returns the next edge in the bubble in the clockwise direction
    fn next_on_bubble(&mut self, key: EdgeKey) -> EdgeKey {
        self.get_edge(key.next_on_vertex()).twin
    }

    // Returns the previous edge in the bubble in the clockwise direction
    fn prev_on_bubble(&mut self, key: EdgeKey) -> EdgeKey {
        self.get_edge(key).twin.next_on_vertex()
    }

    fn get_edge(&mut self, key: EdgeKey) -> &mut Edge {
        &mut self.vertecies[key.vertex].edges[key.offset as usize]
    }
}
