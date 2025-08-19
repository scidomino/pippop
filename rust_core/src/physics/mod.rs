pub mod pressure;
pub mod surface;
pub mod force;

use crate::graph::RelationManager;
use crate::physics::force::Force;


pub fn update_force(relation_manager: &RelationManager) {
    let mut force = Force::new();


    pressure::update_force(relation_manager, &mut force);
    surface::update_force(relation_manager, &mut force);
}