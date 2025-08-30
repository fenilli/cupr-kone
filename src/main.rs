mod core;

use core::{entity_component_set::EntityComponentSet, entity_manager::EntityManager};
use std::any::type_name;

#[derive(Debug, Default)]
struct Position(f32, f32, f32);

fn main() {
    let mut entity_manager = EntityManager::new();
    let mut position_set = EntityComponentSet::<Position>::new();

    let e = entity_manager.allocate();
    println!("Allocated entity: {}", e);

    position_set.insert(e, Position::default());
    println!("Inserted {:?} for entity {}", type_name::<Position>(), e);
    println!("PositionSet: {}", position_set);

    position_set.remove(e);
    println!("PositionSet: {}", position_set);

    position_set.insert(e, Position(1.0, 1.0, 1.0));
    println!("Re-inserted {:?} for entity {}", type_name::<Position>(), e);
    println!("PositionSet: {}", position_set);

    let e2 = entity_manager.allocate();
    println!("Allocated entity: {}", e2);

    position_set.insert(e2, Position(2.0, 2.0, 2.0));
    println!("Inserted {:?} for entity {}", type_name::<Position>(), e2);
    println!("PositionSet: {}", position_set);
}
