// rust_core/src/ffi.rs

use std::ffi::c_void;
use crate::simulation::Simulation;

/// A C-compatible version of our Point struct.
#[repr(C)]
#[derive(Clone)]
pub struct CPoint {
    pub x: f64,
    pub y: f64,
}

/// A C-compatible struct representing a single Bezier curve to be rendered.
#[repr(C)]
#[derive(Clone)]
pub struct CRenderableEdge {
    pub p0: CPoint,
    pub p1: CPoint,
    pub p2: CPoint,
    pub p3: CPoint,
}

/// A struct that holds all the data needed for Unity to render a single frame.
/// It contains pointers to arrays of data. The C# side is responsible for calling
/// `simulation_free_render_state` on this struct to free the memory.
#[repr(C)]
pub struct CRenderState {
    pub edges: *const CRenderableEdge,
    pub edge_count: usize,
    capacity: usize, // Internal capacity, used for freeing memory
}

/// Creates a new simulation instance and returns an opaque pointer to it.
#[no_mangle]
pub extern "C" fn simulation_new() -> *mut c_void {
    let sim = Box::new(Simulation::new());
    Box::into_raw(sim) as *mut c_void
}

/// Frees the memory for a simulation instance.
#[no_mangle]
pub extern "C" fn simulation_free(sim_ptr: *mut c_void) {
    if sim_ptr.is_null() { return; }
    unsafe {
        let _ = Box::from_raw(sim_ptr as *mut Simulation);
    }
}

/// Steps the simulation forward in time.
#[no_mangle]
pub extern "C" fn simulation_update(sim_ptr: *mut c_void, delta_time: f64) {
    if sim_ptr.is_null() { return; }
    let mut sim = unsafe { Box::from_raw(sim_ptr as *mut Simulation) };
    sim.update(delta_time);
    let _ = Box::into_raw(sim);
}

/// Gets the current renderable state of the simulation.
#[no_mangle]
pub extern "C" fn simulation_get_render_state(sim_ptr: *mut c_void) -> CRenderState {
    if sim_ptr.is_null() {
        return CRenderState { edges: std::ptr::null(), edge_count: 0, capacity: 0 };
    }
    let sim = unsafe { &*(sim_ptr as *mut Simulation) };
    
    let mut render_vec: Vec<CRenderableEdge> = Vec::with_capacity(sim.graph.edges.len());
    
    for (_, edge) in sim.graph.edges.iter() {
        let twin = &sim.graph.edges[edge.twin_edge_key];
        let p0 = &sim.graph.vertices[edge.start_vertex_key].position;
        let p1 = &edge.control_point;
        let p2 = &twin.control_point;
        let p3 = &sim.graph.vertices[twin.start_vertex_key].position;

        render_vec.push(CRenderableEdge {
            p0: CPoint { x: p0.x, y: p0.y },
            p1: CPoint { x: p1.x, y: p1.y },
            p2: CPoint { x: p2.x, y: p2.y },
            p3: CPoint { x: p3.x, y: p3.y },
        });
    }
    
    let ptr = render_vec.as_ptr();
    let len = render_vec.len();
    let capacity = render_vec.capacity();
    std::mem::forget(render_vec);

    CRenderState {
        edges: ptr,
        edge_count: len,
        capacity,
    }
}

/// Frees the memory for a render state that was previously returned by `simulation_get_render_state`.
#[no_mangle]
pub extern "C" fn simulation_free_render_state(state: CRenderState) {
    unsafe {
        let _ = Vec::from_raw_parts(
            state.edges as *mut CRenderableEdge,
            state.edge_count,
            state.capacity,
        );
    }
}
