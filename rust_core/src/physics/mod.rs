pub mod pressure;
pub mod surface;
pub mod vector;

// use crate::graph::RelationManager;
// use crate::graph::point::Coordinate;
// use crate::physics::vector::GraphVector;


// pub fn update_force(relation_manager: &RelationManager) {
//     let mut force = GraphVector::new();

//     pressure::update_force(relation_manager, &mut force);
//     surface::update_force(relation_manager, &mut force);

//     for (key, vertex) in relation_manager.vertecies.iter() {
     
//         for edge in vertex.edges.iter() {
//             let (twin, twin_vertex) = relation_manager.get_edge_and_vertex(edge.twin);
//             let edge_key = twin.twin;

//             force.add_edge(
//                 edge_key,
//                 Coordinate {
//                     x: twin.point.position.x - twin_vertex.point.position.x,
//                     y: twin.point.position.y - vertex.point.position.y,
//                 },
//             );
//         }
//     }
// }