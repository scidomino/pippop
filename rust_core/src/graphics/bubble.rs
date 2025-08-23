use crate::graph::{Graph};
use crate::graph::point::Coordinate;
use slotmap::Key;

// --- Public FFI-safe data structures ---

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl From<Coordinate> for Vec2 {
    fn from(c: Coordinate) -> Self {
        Vec2 { x: c.x, y: c.y }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CubicBezier {
    pub p0: Vec2,
    pub p1: Vec2,
    pub p2: Vec2,
    pub p3: Vec2,
}

#[repr(C)]
pub struct RenderableBubble {
    pub bubble_key: u64,
    pub is_open_air: bool,
    pub curves: *mut CubicBezier,
    pub curves_count: usize,
}

#[repr(C)]
pub struct RenderableBubbleCollection {
    pub bubbles: *mut RenderableBubble,
    pub bubbles_count: usize,
}

// --- FFI functions ---

#[no_mangle]
pub extern "C" fn get_renderable_bubbles(graph: &Graph) -> *mut RenderableBubbleCollection {
    let mut renderable_bubbles = Vec::new();

    for (bkey, bubble) in graph.bubbles.iter() {
        let (curves_ptr, curves_len) = if bubble.open_air || bubble.edges.is_empty() {
            (std::ptr::null_mut(), 0)
        } else {
            let mut curves = Vec::with_capacity(bubble.edges.len());
            for &ekey in &bubble.edges {
                let (edge, vertex) = graph.get_edge_and_vertex(ekey);
                let (twin_edge, twin_vertex) = graph.get_edge_and_vertex(edge.twin);

                let bezier = CubicBezier {
                    p0: vertex.point.position.into(),
                    p1: edge.point.position.into(),
                    p2: twin_edge.point.position.into(),
                    p3: twin_vertex.point.position.into(),
                };
                curves.push(bezier);
            }
            let mut curves_vec = curves.into_boxed_slice();
            let ptr = curves_vec.as_mut_ptr();
            let len = curves_vec.len();
            std::mem::forget(curves_vec);
            (ptr, len)
        };

        let renderable = RenderableBubble {
            bubble_key: bkey.data().as_ffi(),
            is_open_air: bubble.open_air,
            curves: curves_ptr,
            curves_count: curves_len,
        };
        renderable_bubbles.push(renderable);
    }

    let mut bubbles_vec = renderable_bubbles.into_boxed_slice();
    let collection = Box::new(RenderableBubbleCollection {
        bubbles: bubbles_vec.as_mut_ptr(),
        bubbles_count: bubbles_vec.len(),
    });
    std::mem::forget(bubbles_vec);

    Box::into_raw(collection)
}

#[no_mangle]
pub unsafe extern "C" fn free_renderable_bubbles(collection_ptr: *mut RenderableBubbleCollection) {
    if collection_ptr.is_null() {
        return;
    }
    let collection = Box::from_raw(collection_ptr);
    let bubbles = std::slice::from_raw_parts_mut(collection.bubbles, collection.bubbles_count);

    for bubble in bubbles {
        if !bubble.curves.is_null() {
            let _ = Vec::from_raw_parts(bubble.curves, bubble.curves_count, bubble.curves_count);
        }
    }

    let _ = Vec::from_raw_parts(collection.bubbles, collection.bubbles_count, collection.bubbles_count);
}
