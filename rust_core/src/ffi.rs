// rust_core/src/ffi.rs

use std::ffi::c_void;
use std::mem;
use crate::simulation::Simulation;
use crate::graph::bubble::BubbleKind;
use slotmap::Key;

#[repr(C)]
#[derive(Clone, Copy)]
pub enum CBubbleKindTag {
    Game,
    Player,
    Empty,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CGameBubbleData {
    pub color: CColor,
    pub size: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union CBubbleKindData {
    pub game: CGameBubbleData,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CPoint {
    pub x: f64,
    pub y: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CBubble {
    pub key_id: u64,
    pub kind_tag: CBubbleKindTag,
    pub kind_data: CBubbleKindData,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRenderableEdge {
    pub p0: CPoint,
    pub p1: CPoint,
    pub p2: CPoint,
    pub p3: CPoint,
    pub bubble_a_key_id: u64,
    pub bubble_b_key_id: u64,
}

#[repr(C)]
pub struct CRenderState {
    pub edges: *mut CRenderableEdge,
    pub edge_count: usize,
    edges_capacity: usize,
    
    pub bubbles: *mut CBubble,
    pub bubble_count: usize,
    bubbles_capacity: usize,
}

#[no_mangle]
pub extern "C" fn simulation_new() -> *mut c_void {
    let sim = Box::new(Simulation::new());
    Box::into_raw(sim) as *mut c_void
}

#[no_mangle]
pub extern "C" fn simulation_free(sim_ptr: *mut c_void) {
    if sim_ptr.is_null() { return; }
    unsafe { let _ = Box::from_raw(sim_ptr as *mut Simulation); }
}

#[no_mangle]
pub extern "C" fn simulation_update(sim_ptr: *mut c_void, delta_time: f64) {
    if sim_ptr.is_null() { return; }
    let mut sim = unsafe { Box::from_raw(sim_ptr as *mut Simulation) };
    sim.update(delta_time);
    let _ = Box::into_raw(sim);
}

#[no_mangle]
pub extern "C" fn simulation_get_render_state(sim_ptr: *mut c_void) -> CRenderState {
    if sim_ptr.is_null() {
        return CRenderState {
            edges: std::ptr::null_mut(), edge_count: 0, edges_capacity: 0,
            bubbles: std::ptr::null_mut(), bubble_count: 0, bubbles_capacity: 0,
        };
    }
    let sim = unsafe { &*(sim_ptr as *mut Simulation) };
    
    let mut bubbles_vec: Vec<CBubble> = Vec::with_capacity(sim.graph.bubbles.len());
    for (key, bubble) in sim.graph.bubbles.iter() {
        let (kind_tag, kind_data) = match bubble.kind {
            BubbleKind::Game { color, size } => (
                CBubbleKindTag::Game,
                CBubbleKindData { game: CGameBubbleData {
                    color: CColor { r: color.r, g: color.g, b: color.b, a: color.a },
                    size,
                }}
            ),
            BubbleKind::Player => (CBubbleKindTag::Player, CBubbleKindData { game: CGameBubbleData { color: CColor{r:0.,g:0.,b:0.,a:0.}, size: 0 } }),
            BubbleKind::Empty => (CBubbleKindTag::Empty, CBubbleKindData { game: CGameBubbleData { color: CColor{r:0.,g:0.,b:0.,a:0.}, size: 0 } }),
        };

        bubbles_vec.push(CBubble {
            key_id: key.data().as_ffi(),
            kind_tag,
            kind_data,
        });
    }

    let mut edges_vec: Vec<CRenderableEdge> = Vec::with_capacity(sim.graph.edges.len() / 2);
    for (_, edge) in sim.graph.edges.iter() {
        if edge.start_vertex_key.data().as_ffi() < edge.twin_edge_key.data().as_ffi() {
            let twin = &sim.graph.edges[edge.twin_edge_key];
            let p0 = &sim.graph.vertices[edge.start_vertex_key].position;
            let p1 = &edge.control_point;
            let p2 = &twin.control_point;
            let p3 = &sim.graph.vertices[twin.start_vertex_key].position;

            edges_vec.push(CRenderableEdge {
                p0: CPoint { x: p0.x, y: p0.y },
                p1: CPoint { x: p1.x, y: p1.y },
                p2: CPoint { x: p2.x, y: p2.y },
                p3: CPoint { x: p3.x, y: p3.y },
                bubble_a_key_id: edge.bubble_key.data().as_ffi(),
                bubble_b_key_id: twin.bubble_key.data().as_ffi(),
            });
        }
    }
    
    let (edges_ptr, edges_len, edges_cap) = {
        let mut v = mem::ManuallyDrop::new(edges_vec);
        (v.as_mut_ptr(), v.len(), v.capacity())
    };
    let (bubbles_ptr, bubbles_len, bubbles_cap) = {
        let mut v = mem::ManuallyDrop::new(bubbles_vec);
        (v.as_mut_ptr(), v.len(), v.capacity())
    };
    
    CRenderState {
        edges: edges_ptr,
        edge_count: edges_len,
        edges_capacity: edges_cap,
        bubbles: bubbles_ptr,
        bubble_count: bubbles_len,
        bubbles_capacity: bubbles_cap,
    }
}

#[no_mangle]
pub extern "C" fn simulation_free_render_state(state: CRenderState) {
    unsafe {
        let _ = Vec::from_raw_parts(state.edges, state.edge_count, state.edges_capacity);
        let _ = Vec::from_raw_parts(state.bubbles, state.bubble_count, state.bubbles_capacity);
    }
}