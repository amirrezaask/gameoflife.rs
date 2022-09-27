use crate::conway::{Cell, CellState};

mod conway;


fn main() {
    let mut world = conway::World::new(4, 4);
    world.forward_time();
    println!("{}", world)
} 
