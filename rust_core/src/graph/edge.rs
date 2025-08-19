use super::bubble::BubbleKey;
use super::point::Point;
use super::vertex::VertexKey;
use core::ops::{Index, IndexMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EdgeKey {
    pub vertex: VertexKey,
    pub offset: u8, // 0, 1, or 2
}

impl EdgeKey {
    pub fn default() -> Self {
        EdgeKey {
            vertex: VertexKey::default(),
            offset: 0,
        }
    }

    pub fn new(vertex: VertexKey, offset: u8) -> Self {
        assert!(offset < 3, "Offset must be 0, 1, or 2");
        EdgeKey { vertex, offset }
    }

    // Returns the next edge key on the vertex in a clockwise direction
    pub fn next_on_vertex(&self) -> EdgeKey {
        EdgeKey {
            vertex: self.vertex,
            offset: (self.offset + 1) % 3,
        }
    }

    // Returns the previous edge key on the vertex in a clockwise direction
    pub fn prev_on_vertex(&self) -> EdgeKey {
        EdgeKey {
            vertex: self.vertex,
            offset: (self.offset + 2) % 3,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    pub point: Point,
    pub twin: EdgeKey,
    pub bubble: BubbleKey,
}

impl Edge {
    pub fn new(point: Point) -> Self {
        Edge {
            point: point,
            twin: EdgeKey::default(),
            bubble: BubbleKey::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct EdgeMap<V> {
    map: slotmap::SecondaryMap<VertexKey, [V; 3]>,
}

impl<V> EdgeMap<V> {
    pub fn new() -> Self {
        EdgeMap {
            map: slotmap::SecondaryMap::new(),
        }
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }

    pub fn get(&self, key: EdgeKey) -> Option<&V> {
        self.map
            .get(key.vertex)
            .map(|edges| &edges[key.offset as usize])
    }

    pub fn get_mut(&mut self, key: EdgeKey) -> Option<&mut V> {
        self.map
            .get_mut(key.vertex)
            .map(|edges| &mut edges[key.offset as usize])
    }

    pub fn insert(&mut self, key: EdgeKey, value: V)
    where
        V: Default,
    {
        self.map
            .entry(key.vertex)
            .unwrap()
            .or_insert(Default::default())[key.offset as usize] = value;
    }
}

impl<V> Index<EdgeKey> for EdgeMap<V> {
    type Output = V;

    fn index(&self, key: EdgeKey) -> &V {
        match self.get(key) {
            Some(r) => r,
            None => panic!("invalid SecondaryMap key used"),
        }
    }
}

impl<V> IndexMut<EdgeKey> for EdgeMap<V> {
    fn index_mut(&mut self, key: EdgeKey) -> &mut V {
        match self.get_mut(key) {
            Some(r) => r,
            None => panic!("invalid SecondaryMap key used"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::vertex::Vertex;
    use super::*;
    use slotmap::SlotMap;

    #[test]
    fn test_edge_map_new() {
        let edge_map: EdgeMap<i32> = EdgeMap::new();
        assert!(edge_map.map.is_empty());
    }

    #[test]
    fn test_edge_map_get_set() {
        let mut sm: SlotMap<VertexKey, Vertex> = SlotMap::with_key();
        let v1 = sm.insert(Vertex::new(Point::new(0.0, 0.0)));
        let mut edge_map: EdgeMap<i32> = EdgeMap::new();
        let edge_key = v1.edge_key(0);

        edge_map.insert(edge_key, 1);

        assert_eq!(edge_map.get(edge_key), Some(&1));
    }

    #[test]
    fn test_edge_map_index() {
        let mut sm: SlotMap<VertexKey, Vertex> = SlotMap::with_key();
        let v1 = sm.insert(Vertex::new(Point::new(0.0, 0.0)));
        let mut edge_map: EdgeMap<i32> = EdgeMap::new();
        let edge_key = EdgeKey::new(v1, 1);

        edge_map.insert(edge_key, 1);

        assert_eq!(edge_map[edge_key], 1);
    }

    #[test]
    fn test_edge_map_index_mut() {
        let mut sm: SlotMap<VertexKey, Vertex> = SlotMap::with_key();
        let v1 = sm.insert(Vertex::new(Point::new(0.0, 0.0)));
        let mut edge_map: EdgeMap<i32> = EdgeMap::new();
        let edge_key = EdgeKey::new(v1, 2);
        edge_map.insert(edge_key, 1);

        edge_map[edge_key] = 10;

        assert_eq!(edge_map[edge_key], 10);
    }
}
